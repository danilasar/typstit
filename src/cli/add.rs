use crate::api::Action;
use crate::cli::Command;

pub struct AddCommand;
impl Command for AddCommand {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Action> {
		todo!()
	}
}