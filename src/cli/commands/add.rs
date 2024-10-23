use crate::api::Request;
use crate::cli::{utils, Command};

pub struct AddCommand;
impl Command for AddCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Request> {
		let package_list = utils::get_package_name_list(
			args,
			true,
			"Usage: add <packages>"
		)?;
		Ok(Request::Add(package_list))
	}
}