use crate::api::Action;
use crate::cli::{utils, Command};

pub struct RemoveCommand;
impl Command for RemoveCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Action> {
		let package_list = utils::get_package_name_list(
			args,
			true,
			"Usage: remove <packages>"
		)?;
		Ok(Action::Remove(package_list))
	}
}