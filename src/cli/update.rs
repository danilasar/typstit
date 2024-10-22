use crate::api::Action;
use crate::cli::Command;

pub struct UpdateCommand;
impl Command for UpdateCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Action> {
		todo!()
	}
}