use env_logger;
mod cli;
mod git;
mod api;

fn main() {
    env_logger::init();
    let action = cli::get_action();
    println!("{:#?}", action);
}
