use neovim_lib::{Neovim, NeovimApi, Session};
use markdownizer::Markdownizer;

struct EventHandler {
    nvim: Neovim,
    markdownizer: Markdownizer,
}

impl EventHandler {
    fn new() -> EventHandler {
        let mut session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        let markdownizer = Markdownizer::new("~/think/todo/projets/");
        EventHandler { nvim, markdownizer }
    }

    // Handle events
    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, values) in receiver {
            match Messages::from(event) {
                Messages::ProjectList => {
                    let plist = self.markdownizer.project_list("~/think/todo/projets/");
                    self.nvim // <-- Echo response to Nvim
                        .command(&format!("echo \"Project list: {}\"", plist.to_string()))
                        .unwrap();
                }
                Messages::Unknown(uevent) => {
                    // unknown event
                }
            }
        }
    }
}
