use crate::api::Action;
use crate::cli::Command;
use crate::cli::utils::get_package_name_list;

pub struct UpdateCommand;
impl Command for UpdateCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Action> {
		let packages_list = get_package_name_list(args, false,"Usage: update [packages]")?;
		if packages_list.0.is_empty() {
			return Ok(Action::UpdateAll)
		}
		Ok(Action::Update(packages_list))
	}
}