use crate::api::Action;
use crate::cli::Command;
use crate::cli::utils::get_package_name_list;

pub struct InfoCommand;
impl Command for InfoCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Action> {
		let packages_list = get_package_name_list(args, true, "Usage: info <package>")?;
		if packages_list.0.len() > 1 {
			log::error!("Too much packages!");
			return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
		}
		Ok(Action::Info(packages_list.0[0].clone()))
	}
}