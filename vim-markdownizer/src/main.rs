mod eventhandler;
mod messages;

fn main() {
    let mut nvim = eventhandler::EventHandler::new();

    nvim.handle_events();
}
