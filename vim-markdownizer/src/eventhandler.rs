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
                Messages::Dashboard => {
                   // let buf = values.pop().unwrap().into();
                   let buf = self.nvim.get_current_buf().unwrap();
                   buf.set_lines(&mut self.nvim, 0, -1, true, vec!("in".into(), "dashboard".into())).unwrap();
                   //Open dashboard pane
                    //  see https://github.com/rafi/vim-sidemenu/blob/master/autoload/sidemenu.vim
                    //Show dashboard content
                    //  - markdownizer.construct_data
                    //  - vim buffer display data
                },
                Messages::ProjectList => {
                    // let curr_dir: PathBuf = self.nvim.command_output("echo expand('%:p:h')").unwrap().into();
                    // let curr_dir: PathBuf = self.vim_ask("expand", vec!("%:p:h")).unwrap().into();
                    let curr_dir: PathBuf = self.vim_ask("expand('%:p:h')").unwrap().into();
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

    // fn get_project_list(&mut self) -> Result<Vec<String>, markdownizer::MarkdownizerError> {
    //     let curr_dir: PathBuf = self.vim_ask("expand('%:p:h')").unwrap().into();
    //     let result = self.markdownizer.project_list();
    //     result.and_then(|plist| 
    //             plist.into_iter().map(|stored_project| {
    //                 let project = &stored_project.entity;
    //                 let location = &stored_project.location;
    //                 let relative_path = diff_paths(location, &curr_dir).unwrap();
    //                 format!("[{}]({}) ({})", project.title, relative_path.to_str().unwrap(), project.tasks.len())
    //             }).collect())
    // }

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
