use neovim_lib::{Neovim, NeovimApi, Session};
use pathdiff::diff_paths;
use markdownizer::Markdownizer;
use crate::messages::Messages;
use std::path::PathBuf;

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
                    // let curr_dir: PathBuf = self.nvim.command_output("echo expand('%:p:h')").unwrap().into();
                    let curr_dir: PathBuf = self.nvim.call_function("expand", vec!("%:p:h".into()))
                        .map(|val| String::from( val.as_str().unwrap() ))
                        .unwrap().into();
                    let result = self.markdownizer.project_list();
                    match result {
                        Ok(plist) => {
                            let plist_str = plist.into_iter().map(|stored_project| {
                                let project = &stored_project.entity;
                                let location = &stored_project.location;
                                let relative_path = diff_paths(location, &curr_dir).unwrap();
                                format!("[{}]({}) ({})", project.title, relative_path.to_str().unwrap(), project.tasks.len())
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
