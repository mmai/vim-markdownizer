use neovim_lib::{Neovim, NeovimApi, Session};
use markdownizer::Markdownizer;
use crate::messages::Messages;

pub struct EventHandler {
    nvim: Neovim,
    markdownizer: Markdownizer,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        let mut session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);


        let proot = std::path::PathBuf::from("~/think/todo/projets/");
        let markdownizer = Markdownizer::new(&proot);
        EventHandler { nvim, markdownizer }
    }

    // Handle events
    pub fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, values) in receiver {
            match Messages::from(event) {
                Messages::ProjectList => {
                    let plist = self.markdownizer.project_list().unwrap();

                    for entry in plist {
                        match entry {
                            project => {
                                self.nvim // <-- Echo response to Nvim
                                    .command(&format!("echo \"Project: {} ({})\"", project.title, project.tasks.len()))
                                    .unwrap();
                            },
                            e => {
                                self.nvim // <-- Echo response to Nvim
                                    .command(&format!("echo \"Not a project: {:?} \"", e))
                                    .unwrap();
                            }
                        }
                    }
                }
                Messages::Unknown(uevent) => {
                    // unknown event
                }
            }
        }
    }
}
