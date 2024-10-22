use crate::api::Action;
use crate::cli::Command;

pub struct UninstallCommand;

impl Command for UninstallCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Action> {
		todo!()
	}
}