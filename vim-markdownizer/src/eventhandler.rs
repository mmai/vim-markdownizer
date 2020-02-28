use neovim_lib::{Neovim, NeovimApi, Session};
use markdownizer::Markdownizer;
use crate::messages::Messages;

pub struct EventHandler {
    nvim: Neovim,
    markdownizer: Markdownizer,
}

impl EventHandler {
    pub fn new(root: &str) -> EventHandler {
        let mut session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        let proot = std::path::PathBuf::from(root);
        let markdownizer = Markdownizer::new(&proot);
        EventHandler { nvim, markdownizer }
    }

    // Handle events
    pub fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, values) in receiver {
            match Messages::from(event) {
                Messages::ProjectList => {
                    let result = self.markdownizer.project_list();
                    match result {
                        Ok(plist) => {
                            let plist_str = plist.into_iter().map(|project| {
                                format!("{} ({})", project.title, project.tasks.len())
                            }).collect();
                            self.nvim.put(plist_str, "", true, true).unwrap();
                            // self.obsolete_put(plist_str);
                        },
                        Err(e) => {
                            self.nvim.err_writeln(&format!("Error when reading projects : {}", e)).unwrap();
                        }
                    }
                }
                Messages::Unknown(uevent) => {
                    // unknown event
                }
            }
        }
    }

    fn obsolete_put(&mut self, plist_str: Vec<String>) {
        // Before put was available :
        let win = self.nvim.get_current_win().unwrap();
        let (row, _col) = win.get_cursor(&mut self.nvim).unwrap();
        let buf = self.nvim.get_current_buf().unwrap();
        buf.set_lines(&mut self.nvim, row, row, true, plist_str).unwrap();
    }
}
