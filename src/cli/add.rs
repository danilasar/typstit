use crate::api::Action;
use crate::cli::{utils, Command};

pub struct AddCommand;
impl Command for AddCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Action> {
		let package_list = utils::get_package_name_list(
			args,
			true,
			"Usage: add <packages>"
		)?;
		Ok(Action::Add(package_list))
	}
}