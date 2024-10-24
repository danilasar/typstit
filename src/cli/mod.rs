mod utils;
mod commands;

use std::collections::HashMap;
use crate::api::Request;
use commands::{
	add::AddCommand,
	info::InfoCommand,
	install::InstallCommand,
	remove::RemoveCommand,
	uninstall::UninstallCommand,
	update::UpdateCommand,
	help::HelpCommand
};
//use crate::cli::{Add, Help, Install, Remove, Uninstall, Update};

pub trait Command {
	fn execute(&self, args: Vec<String>) -> std::io::Result<Request>;
}

pub fn get_action<'a>() -> std::io::Result<Request> {
	let mut args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		println!("Please specify a command: typstit <command>");
		args.push(String::from("help"));
	}
	let action = &args[1];
	let actions : HashMap<&str, Box<dyn Command>> = HashMap::from([
		("help",      Box::new(HelpCommand)      as Box<dyn Command>),
		("install",   Box::new(InstallCommand)   as Box<dyn Command>),
		("uninstall", Box::new(UninstallCommand) as Box<dyn Command>),
		("add",       Box::new(AddCommand)       as Box<dyn Command>),
		("remove",    Box::new(RemoveCommand)    as Box<dyn Command>),
		("update",    Box::new(UpdateCommand)    as Box<dyn Command>),
		("info",      Box::new(InfoCommand)      as Box<dyn Command>)
	]);
	for act in actions.iter() {
		if act.0 != action {
			continue
		}
		return act.1.execute(args);
	}
	println!("Undefined command: {}", &action);
	Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
}