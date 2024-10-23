use crate::api::Request;
use crate::cli::utils;

pub struct InstallCommand;
impl crate::cli::Command for InstallCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Request> {
		let package_list = utils::get_package_name_list(
			args,
			true,
			"Usage: install <packages>"
		)?;
		Ok(Request::Install(package_list))
	}
}