/*
 * ДИСКЛЕЙМЕР
 * Код использует инструментарий утилиты git,
 * так как в libgit2 пока не реализована поддержка
 * sparse checkout. Как только она появится,
 * необходимость в этом костыле отпадёт.
 *
 * В любом случае в ближайшее время будет проведён крупный рефакторинг,
 * так как автор кода --- не я, а японский автор писал его, мягко говоря,
 * спустя рукава.
 */

use cmd_lib::run_cmd;
use std::path::{Path, PathBuf};

pub fn repo(path: impl Into<String>) -> Downloader {
	Downloader::new(path)
}

#[derive(Debug)]
struct CopyRequest {
	from: PathBuf,
	to: PathBuf,
}
#[derive(Debug)]
pub struct Downloader {
	repo_path: String,
	branch_name: String,
	out_dir: PathBuf,
	copy_requests: Vec<CopyRequest>,
}
impl Downloader {
	fn new(repo: impl Into<String>) -> Self {
		let cur_dir = std::env::current_dir().unwrap();
		Self {
			repo_path: repo.into(),
			branch_name: "master".to_owned(),
			out_dir: cur_dir,
			copy_requests: vec![],
		}
	}
	/// Изменяет папку, куда сохраняется файл
	pub fn out_dir(mut self, path: impl AsRef<Path>) -> Self {
		self.out_dir = path.as_ref().to_owned();
		self
	}
	/// Переключаемся между ветками, по умолчанию смотрим в "master".
	pub fn branch_name(mut self, name: impl Into<String>) -> Self {
		self.branch_name = name.into();
		self
	}
	/// Добавляем файл для копирования на локальную машину.
	/// Путь `src` указывается относительно корня репозитория и
	/// путь `dst` --- относительно `out_dir`.
	pub fn add_file(mut self, src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Self {
		let from = src.as_ref().to_owned();
		let to = dst.as_ref().to_owned();
		let req = CopyRequest { from, to };
		self.copy_requests.push(req);
		self
	}
	/// Выполняем загрузку
	pub fn exec(self) -> anyhow::Result<()> {
		let old_pwd = std::env::current_dir()?;

		let dir = tempfile::tempdir()?;
		let dir_path = dir.path();
		std::env::set_current_dir(dir_path)?;

		let repo = &self.repo_path;
		run_cmd! {
            git init .;
            git config core.sparsecheckout true;
            git remote add origin $repo;
        };

		for req in &self.copy_requests {
			let from = &req.from;
			run_cmd! {
                echo $from >> .git/info/sparse-checkout;
            };
		}

		let branch_name = &self.branch_name;
		run_cmd! {
            git pull origin $branch_name;
        };

		for req in &self.copy_requests {
			let from = &req.from;
			let to = &req.to;
			let to = self.out_dir.join(to);
			let to_dir = to.parent().unwrap();
			if !to_dir.exists() {
				std::fs::create_dir_all(to_dir)?;
			}
			run_cmd! {
                mv $from $to;
            };
		}

		std::env::set_current_dir(old_pwd)?;
		Ok(())
	}
}