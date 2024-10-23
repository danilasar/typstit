use crate::api::Request;
use crate::cli::Command;

pub struct HelpCommand;
impl Command for HelpCommand {
	fn execute(&self, _: Vec<String>) -> std::io::Result<Request> {
		println!("typstit help - print this help");
		println!("typstit install <packages> - install packages to local storage");
		println!("typstit uninstall <packages> - uninstall packages from local storage");
		println!("typstit add <doc.typ> <packages> - import packages into document");
		println!("typstit remove <package> - remove packages from document");
		println!("typstit update [packages] - update specific packages (all by default)");
		println!("typstit info <package> - print package info");
		Ok(Request::Nothing)
	}
}