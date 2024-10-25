mod package;

use std::io::Read;
use std::path::{Path, PathBuf};
use log::error;
use rand::{random, Rng};
use crate::api::{PackageName, Request, Response};
use crate::core::package::Package;

pub struct Typstit;
impl Typstit {

	pub fn execute(request: Request) -> Response {

		match request {
			Request::Info(package)      => Self::info(package),
			Request::Install(_)   => Self::install(request),
			Request::Update(_)    => Self::update_package(request),
			Request::UpdateAll    => Self::update_all(request),
			Request::Add(_)       => Self::add(request),
			Request::Remove(_)    => Self::remove(request),
			Request::Uninstall(_) => Self::uninstall(request),
			Request::Nothing      => Response::Nothing
		}
	}

	pub fn info(package: PackageName) -> Response {
		let mut file = match tempfile::tempdir() {
			Ok(dir) => dir.into_path(),
			Err(e) => {
				error!("Can't get temp dir: {}", e);
				return Response::Nothing
			}
		};
		let seed = random::<u64>().to_string();
		let hash_str = format!("{:x}", md5::compute(seed));
		file.push(std::process::id().to_string() + "_" + hash_str.as_str());
		match crate::git::download_file::repo(package.0)
			.add_file("Typstit.toml", file.clone())
			.exec() {
			Ok(_)  => {
				let mut f = match std::fs::File::open(file) {
					Ok(f) => f,
					Err(e) => {
						error!("Can't open file: {}", e);
						return Response::Nothing;
					}
				};
				let mut str_buf = String::new();
				if let Err(e) = f.read_to_string(&mut str_buf) {
					error!("Can't read file: {}", e);
					return Response::Nothing;
				};
				println!("{}", str_buf); // todo parsing
				Response::Nothing
			},
			Err(_) => {
				error!("Can't get Typstit.toml file from repo");
				Response::Nothing
			}
		}
	}

	pub fn add(request: Request) -> Response {
		todo!()
	}

	pub fn remove(request: Request) -> Response {
		todo!()
	}

	pub fn install(request: Request) -> Response {
		todo!()
	}

	pub fn uninstall(request: Request) -> Response {
		todo!()
	}

	pub fn update_package(request: Request) -> Response {
		todo!()
	}

	pub fn update_all(_: Request) -> Response {
		todo!()
	}

}
