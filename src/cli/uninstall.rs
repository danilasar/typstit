use crate::api::Action;
use crate::cli::{utils, Command};

pub struct UninstallCommand;

impl Command for UninstallCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Action> {
		let package_list = utils::get_package_name_list(
			args,
			true,
			"Usage: uninstall <packages>"
		)?;
		Ok(Action::Install(package_list))
	}
}