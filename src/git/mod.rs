pub mod download_file;

use std::path::PathBuf;
use git2::Repository;
use crate::git::GitError::{*};

#[derive(Debug)]
pub enum GitError {
	RepoNotFound(git2::Error),
	RemoteNotFound(String),
	BranchNotFound(String),
	CommitNotFound(git2::Error),
	ReferenceNotFound(git2::Error),
	CommitFetchError(git2::Error),
	MergeAnalysisError(git2::Error),
	TargetNotChanged(git2::Error),
	HeadNotChanged(git2::Error),
	CheckoutError(git2::Error),
	FastForwardError,
	OtherInternalError(git2::Error)
}

fn get_tree_by_branch<'a>(repo: &'a git2::Repository, branch: &str) -> Result<git2::Tree<'a>, GitError> {
	let object = match repo.revparse_single(branch) {
		Ok(obj) => obj,
		Err(e) => return Err(BranchNotFound(branch.to_string()))
	};
	let commit = match object.peel_to_commit() {
		Ok(cmt) => cmt,
		Err(e) => return Err(CommitNotFound(e))
	};
	match commit.tree() {
		Ok(tree) => Ok(tree),
		Err(e) => return Err(OtherInternalError(e))
	}
}


pub fn load_repository(path : &std::path::Path) -> Result<git2::Repository, GitError> {
	match git2::Repository::open(path) {
		Ok(r) => Ok(r),
		Err(e) => Err(RepoNotFound(e))
	}
}

fn fetch_updates(repo : &Repository) -> Result<(), GitError> {
	let mut callbacks = git2::RemoteCallbacks::new();
	callbacks.credentials(|_url, username_from_url, _allowed_types| {
		git2::Cred::ssh_key(
			username_from_url.unwrap(),
			None,
			std::path::Path::new(&format!("{}/.ssh/tutors-deploy", std::env::var("HOME").unwrap())),
			None,
		)
	});

	let mut fo = git2::FetchOptions::new();
	fo.remote_callbacks(callbacks);

	match repo.find_remote("origin") {
		Ok(mut remote) => if let Err(e) = remote.fetch(&["master"], Some(&mut fo), None) {
			log::info!("{:#?}", e);
			return Err(BranchNotFound("origin/master".to_string()))
		},
		Err(_) => return Err(RemoteNotFound("origin".to_string()))
	};
	Ok(())
}

fn get_changed_files(repo: &Repository) -> Result<Vec<(git2::Delta, std::path::PathBuf, std::path::PathBuf)>, GitError> {
	let local = get_tree_by_branch(&repo, "master")?;
	let remote = get_tree_by_branch(&repo, "origin/master")?;
	let diff = match repo.diff_tree_to_tree(Some(&local), Some(&remote), Some(&mut git2::DiffOptions::new())) {
		Ok(diff) => diff,
		Err(e) => return Err(OtherInternalError(e)),
	};

	let mut changed_files : Vec<(git2::Delta, std::path::PathBuf, std::path::PathBuf)> = vec![];

	for delta in diff.deltas() {
		let repo_path = std::path::Path::new("static/repo").to_path_buf();
		let new_file = match delta.new_file().path() {
			Some(file) => file.to_path_buf(),
			None => continue
		};
		let new_file = repo_path.join(new_file);
		let old_file = match delta.old_file().path() {
			Some(file) => repo_path.join(file),
			None => continue
		};
		let status = delta.status();
		let allowed_statuses = [git2::Delta::Added, git2::Delta::Deleted, git2::Delta::Modified, git2::Delta::Renamed];
		if allowed_statuses.contains(&status) {
			changed_files.push((status, new_file, old_file));
		}
	}

	Ok(changed_files)
}

fn get_filename(path_buf: &PathBuf) -> Result<&str, ()> {
	match path_buf.file_name() {
		Some(filename) => match filename.to_str() {
			Some(filename) => Ok(filename),
			None => Err(())
		},
		None => Err(())
	}
}

fn remove_newline(s: &mut String) {
	if s.ends_with('\n') {
		s.pop();
		if s.ends_with('\r') {
			s.pop();
		}
	}
}


pub async fn sync_packages(repo_path: PathBuf) -> Result<(), GitError> {
	let repo = load_repository(repo_path.as_path())?;
	fetch_updates(&repo)?;
	let _ = get_changed_files(&repo)?;
	fast_forward(&repo)?;
	Ok(())
}

fn fast_forward(repo : &Repository) -> Result<(), GitError> {
	let fetch_head = match repo.find_reference("FETCH_HEAD") {
		Ok(r) => r,
		Err(e) => return Err(ReferenceNotFound(e))
	};
	let fetch_commit = match repo.reference_to_annotated_commit(&fetch_head) {
		Ok(c) => c,
		Err(e) => return Err(CommitFetchError(e))
	};

	let analysis = match repo.merge_analysis(&[&fetch_commit]) {
		Ok(an) => an,
		Err(e) => return Err(MergeAnalysisError(e))
	};
	if analysis.0.is_up_to_date() {
		Ok(())
	} else if analysis.0.is_fast_forward() {
		let refname = format!("refs/heads/{}", "master");
		let mut reference = match repo.find_reference(&refname) {
			Ok(r) => r,
			Err(e) => return Err(ReferenceNotFound(e))
		};

		if let Err(e) = reference.set_target(fetch_commit.id(), "Fast-Forward") {
			return Err(TargetNotChanged(e));
		}
		if let Err(e) = repo.set_head(&refname) {
			return Err(HeadNotChanged(e));
		}
		if let Err(e) = repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force())) {
			return Err(CheckoutError(e));
		}
		Ok(())
	} else {
		Err(FastForwardError)
	}
}