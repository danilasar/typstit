mod package;

use crate::api::{Request, Response};

pub struct Typstit;
impl Typstit {

	pub fn execute(request: Request) -> Response {

		match request {
			Request::Info(_)      => Self::info(request),
			Request::Install(_)   => Self::install(request),
			Request::Update(_)    => Self::update_package(request),
			Request::UpdateAll    => Self::update_all(request),
			Request::Add(_)       => Self::add(request),
			Request::Remove(_)    => Self::remove(request),
			Request::Uninstall(_) => Self::uninstall(request),
			Request::Nothing      => Response::Nothing,
			_ => todo!()
		}
	}

	pub fn info(request: Request) -> Response {
		todo!()
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
