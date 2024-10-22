use crate::api::Action;
use crate::cli::Command;

pub struct RemoveCommand;
impl Command for RemoveCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Action> {
		todo!()
	}
}