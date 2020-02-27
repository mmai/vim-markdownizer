use std::env;

mod eventhandler;
mod messages;

fn main() {
    let args: Vec<String> = env::args().collect();
    let projects_dir = &args[1]; // projects files directory
    let mut nvim = eventhandler::EventHandler::new(projects_dir);
    nvim.recv();
}
