use env_logger;
use crate::api::Request;

mod cli;
mod git;
mod api;
mod core;

fn main() {
    env_logger::init();
    let action = cli::get_action();
    match action {
        Err(e) => log::error!("{}", e),
        Ok(request) => {
            let _ = core::Typstit::execute(request);
        }
    }
}
