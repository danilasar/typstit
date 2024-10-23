use crate::api::Request;
use crate::cli::{utils, Command};

pub struct UninstallCommand;

impl Command for UninstallCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Request> {
		let package_list = utils::get_package_name_list(
			args,
			true,
			"Usage: uninstall <packages>"
		)?;
		Ok(Request::Install(package_list))
	}
}