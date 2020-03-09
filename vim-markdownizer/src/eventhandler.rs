use neovim_lib::{Neovim, NeovimApi, Session};
use pathdiff::diff_paths;
use markdownizer::{types, Markdownizer};
use crate::messages::Messages;
use std::path::PathBuf;

type StoredProject = types::Stored<types::Project>;

pub struct State {
    loaded: bool,
    projects: Vec<StoredProject>
}

pub struct EventHandler {
    nvim: Neovim,
    state: State,
    markdownizer: Markdownizer,
}

impl EventHandler {
    pub fn new(root: &str) -> EventHandler {
        let mut session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        let state = State { loaded: false, projects: vec!() };
        let proot = std::path::PathBuf::from(root);
        let markdownizer = Markdownizer::new(&proot);
        EventHandler { nvim, state, markdownizer }
    }

    // Handle events
    pub fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, mut values) in receiver {
            match Messages::from(event) {
                Messages::Dashboard => {
                   // let buf = values.pop().unwrap().into();
                   let buf = self.nvim.get_current_buf().unwrap();
                   if (! &self.state.loaded){
                       &self.init_state();
                   }
                   let plist = self.state.projects.iter().map(|p| String::from(&p.entity.title)).collect();
                   buf.set_lines(&mut self.nvim, 0, -1, true, plist).unwrap();
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
                   let win_id = values.pop().unwrap().as_i64().unwrap();

                   let curr_dir: PathBuf = self.vim_ask("expand('%:p:h')").unwrap().into();
                   let stored_project:&StoredProject = &self.state.projects[line as usize - 1];
                   // let stored_project:&StoredProject = self.state.projects.get(line.into()).unwrap();
                   let project = &stored_project.entity;
                   let location = &stored_project.location;
                   let relative_path = diff_paths(location.as_path(), &curr_dir).unwrap();
                   let file_path = String::from(format!("{}", relative_path.to_str().unwrap()));


                   let target_win = self.get_window(win_id);
                   self.nvim.set_current_win(&target_win).unwrap();
                   self.nvim.command(&format!("echo '{}'", file_path)).unwrap();
                   self.nvim.command(&format!("e {}", file_path)).unwrap();
                   // self.nvim.err_writeln(&format!("{}", file_path)).unwrap();
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

    fn init_state(&mut self) {
        let result = self.markdownizer.project_list();
        self.state.projects = result.unwrap_or(vec!());
    }

    // Call a vim function which return output
    // fn vim_ask(&mut self, func: &str, params: Vec<&str>) -> Result<String, neovim_lib::neovim::CallError> {
        // self.nvim.call_function(func, params.into_iter().map(|v| v.into()).collect())
    fn vim_ask(&mut self, expr: &str) -> Result<String, neovim_lib::neovim::CallError> {
        self.nvim.eval(expr)
            .map(|val| String::from( val.as_str().unwrap() ))
    }

    fn get_window(&mut self, id: i64) -> neovim_lib::neovim_api::Window {
        self.nvim.get_current_win().unwrap()
    }

    fn obsolete_put(&mut self, plist_str: Vec<String>) {
        // Before put was available :
        let win = self.nvim.get_current_win().unwrap();
        let (row, _col) = win.get_cursor(&mut self.nvim).unwrap();
        let buf = self.nvim.get_current_buf().unwrap();
        buf.set_lines(&mut self.nvim, row, row, true, plist_str).unwrap();
    }
}
