use crate::api::Action;
use crate::cli::utils;

pub struct InstallCommand;
impl crate::cli::Command for InstallCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Action> {
		let package_list = utils::get_package_name_list(
			args,
			true,
			"Usage: install <packages>"
		)?;
		Ok(Action::Install(package_list))
	}
}