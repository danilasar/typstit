use crate::api::Request;
use crate::cli::Command;
use crate::cli::utils::get_package_name_list;

pub struct InfoCommand;
impl Command for InfoCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Request> {
		let packages_list = get_package_name_list(args, true, "Usage: info <package>")?;
		if packages_list.0.len() > 1 {
			log::error!("Too much packages!");
			return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
		}
		Ok(Request::Info(packages_list.0[0].clone()))
	}
}