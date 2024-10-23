use crate::api::Request;
use crate::cli::{utils, Command};

pub struct RemoveCommand;
impl Command for RemoveCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Request> {
		let package_list = utils::get_package_name_list(
			args,
			true,
			"Usage: remove <packages>"
		)?;
		Ok(Request::Remove(package_list))
	}
}