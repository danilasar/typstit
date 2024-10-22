use crate::api::Action;

pub struct InstallCommand;
impl crate::cli::Command for InstallCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Action> {
		todo!()
	}
}