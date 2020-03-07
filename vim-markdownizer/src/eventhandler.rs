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

        for (event, mut values) in receiver {
            match Messages::from(event) {
                Messages::Dashboard => {
                   // let buf = values.pop().unwrap().into();
                   let buf = self.nvim.get_current_buf().unwrap();
                   let result = self.get_project_list();
                   match result {
                     Ok(plist) => {
                       buf.set_lines(&mut self.nvim, 0, -1, true, plist).unwrap();
                     },
                     Err(e) => {
                       self.nvim.err_writeln(&format!("Error when reading projects : {}", e)).unwrap();
                     }
                   }
                },
                Messages::ProjectList => {
                    let result = self.get_project_list();
                    match result {
                        Ok(plist) => {
                            self.nvim.put(plist, "", true, true).unwrap();
                            // self.obsolete_put(plist_str);
                        },
                        Err(e) => {
                            self.nvim.err_writeln(&format!("Error when reading projects : {}", e)).unwrap();
                        }
                    }
                }
                Messages::ProjectSelect => {
                   let line = values.pop().unwrap().as_i64().unwrap();
                   let win_content = values.pop().unwrap().as_i64().unwrap();

                   let buf = self.nvim.get_current_buf().unwrap();
                   let project_str = buf.get_lines(&mut self.nvim, line, line + 1, true).unwrap();
                   self.nvim.err_writeln(&format!("{:?}", project_str)).unwrap();
                    // let result = self.get_project_file();
                    // match result {
                    //     Ok(file) => {
                    //         self.nvim.put(plist, "", true, true).unwrap();
                    //         // self.obsolete_put(plist_str);
                    //     },
                    //     Err(e) => {
                    //         self.nvim.err_writeln(&format!("Error when reading projects : {}", e)).unwrap();
                    //     }
                    // }
                }
                Messages::Unknown(uevent) => {
                    // unknown event
                }
            }
        }
    }

    fn get_project_list(&mut self) -> Result<Vec<String>, markdownizer::MarkdownizerError> {
        // let curr_dir: PathBuf = self.nvim.command_output("echo expand('%:p:h')").unwrap().into();
        // let curr_dir: PathBuf = self.vim_ask("expand", vec!("%:p:h")).unwrap().into();
        let curr_dir: PathBuf = self.vim_ask("expand('%:p:h')").unwrap().into();
        let result = self.markdownizer.project_list();
        result.and_then(|plist| {
          let lines = plist.into_iter().map(|stored_project| {
            let project = &stored_project.entity;
            let location = &stored_project.location;
            let relative_path = diff_paths(location, &curr_dir).unwrap();
            String::from(format!("[{}]({}) ({})", project.title, relative_path.to_str().unwrap(), project.tasks.len()))
          }).collect();
          Ok(lines)
        })
    }

    // Call a vim function which return output
    // fn vim_ask(&mut self, func: &str, params: Vec<&str>) -> Result<String, neovim_lib::neovim::CallError> {
        // self.nvim.call_function(func, params.into_iter().map(|v| v.into()).collect())
    fn vim_ask(&mut self, expr: &str) -> Result<String, neovim_lib::neovim::CallError> {
        self.nvim.eval(expr)
            .map(|val| String::from( val.as_str().unwrap() ))
    }

    fn obsolete_put(&mut self, plist_str: Vec<String>) {
        // Before put was available :
        let win = self.nvim.get_current_win().unwrap();
        let (row, _col) = win.get_cursor(&mut self.nvim).unwrap();
        let buf = self.nvim.get_current_buf().unwrap();
        buf.set_lines(&mut self.nvim, row, row, true, plist_str).unwrap();
    }
}
