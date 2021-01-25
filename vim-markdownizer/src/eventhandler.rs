use async_trait::async_trait;
use tokio::io::Stdout;
use rmpv::Value;

use pathdiff::diff_paths;
use markdownizer::{types, Markdownizer};
use std::path::PathBuf;

use nvim_rs::{
  compat::tokio::Compat, Handler, Neovim, Window
};


type StoredProject = types::Stored<types::Project>;

#[derive(Clone)]
pub struct State {
    loaded: bool,
    // content_win: Option<Window>,
    projects: Vec<StoredProject>
}

#[derive(Clone)]
pub struct NeovimHandler{
    state: State,
    markdownizer: Markdownizer,
}

impl NeovimHandler {
    pub fn new(projects_dir: &str) -> Self {
        NeovimHandler {
            state: State { loaded: false, projects: vec![] },
            markdownizer: Markdownizer::new(&projects_dir.into())
        }
    }
}

#[async_trait]
impl Handler for NeovimHandler {
  type Writer = Compat<Stdout>;

  async fn handle_request(
    &self,
    name: String,
    _args: Vec<Value>,
    neovim: Neovim<Compat<Stdout>>,
  ) -> Result<Value, Value> {
    match name.as_ref() {
      "init_content_window" => {
          neovim.command("echom 'some win value=..'").await.unwrap();
          // neovim.command(&format!("echom 'some win value={:?}'", win)).unwrap();
        // let c = neovim.get_current_buf().await.unwrap();
        // for _ in 0..1_000_usize {
        //   let _x = c.get_lines(0, -1, false).await;
        // }
        Ok(Value::Nil)
      },
      // "buffer" => {
      //   for _ in 0..10_000_usize {
      //     let _ = neovim.get_current_buf().await.unwrap();
      //   }
      //   Ok(Value::Nil)
      // },
      // "api" => {
      //   for _ in 0..1_000_usize {
      //     let _ = neovim.get_api_info().await.unwrap();
      //   }
      //   Ok(Value::Nil)
      // },
      _ => Ok(Value::Nil)
    }
  }
}


// use neovim_lib::{Neovim, NeovimApi, Session, Value, Handler, RequestHandler};
// use neovim_lib::neovim_api::{Window};
// use crate::messages::Messages;

/*
struct MyHandler {
    event_handler: &'static EventHandler
}

impl Handler for MyHandler {}
impl RequestHandler for MyHandler {
    fn handle_request(&mut self, _name: &str, _args: Vec<Value>) -> Result<Value, Value> {
        self.event_handler.nvim.command(&format!("echom 'in handle_request'")).unwrap();
        Ok(Value::from(true))
    }
}

impl EventHandler {
    pub fn new(root: &str) -> EventHandler {
        let mut session = Session::new_parent().unwrap();
        let mut nvim = Neovim::new(session);
        let state = State { loaded: false, content_win: None, projects: vec!() };
        let proot = std::path::PathBuf::from(root);
        let markdownizer = Markdownizer::new(&proot);
        EventHandler { nvim, state, markdownizer }
    }

    // Handle events
    pub fn recv(&mut self) {
        let my_handler = MyHandler { event_handler: self };
        let receiver = self.nvim.session.start_event_loop_channel_handler(my_handler);

        for (event, mut values) in receiver {
            match Messages::from(event) {
                Messages::InitContentWindow => {
                    let content_win = self.nvim.get_current_win().unwrap();
                    self.state.content_win = Some(content_win.clone());
                    self.nvim.command(&format!("echom 'initwin value={:?}'", content_win)).unwrap();

                    let wins = self.nvim.list_wins().unwrap();
                    let _:() = wins.iter().map(|win| {
                        self.nvim.command(&format!("echom 'some win value={:?}'", win)).unwrap();
                        ()
                    }).collect();
                },
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
                   let win_id = values.pop().unwrap();

                   let curr_dir: PathBuf = self.vim_ask("expand('%:p:h')").unwrap().into();
                   let stored_project:&StoredProject = &self.state.projects[line as usize - 1];
                   // let stored_project:&StoredProject = self.state.projects.get(line.into()).unwrap();
                   let project = &stored_project.entity;
                   let location = &stored_project.location;
                   let relative_path = diff_paths(location.as_path(), &curr_dir).unwrap();
                   let file_path = String::from(format!("{}", relative_path.to_str().unwrap()));

                   let cwin = self.state.content_win.clone().unwrap();
                   self.nvim.set_current_win(&cwin).unwrap();
                   // self.nvim.command(&format!("echo '{}'", file_path)).unwrap();
                   self.nvim.command(&format!("e {}", file_path)).unwrap();
                   // self.nvim.err_writeln(&format!("{}", file_path)).unwrap();
                }
                Messages::Unknown(uevent) => {
                   self.nvim.err_writeln(&format!("unkown event {:?}", uevent)).unwrap();
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

    fn get_window(&mut self, id: Value) -> neovim_lib::neovim_api::Window {
        // Window::new(id)
        let curwin = self.nvim.get_current_win().unwrap();
        let win_id = curwin.get_value();
        self.nvim.command(&format!("echom 'curwin value={:?}'", win_id)).unwrap();



        let wins = self.nvim.list_wins().unwrap();
        let _:() = wins.iter().map(|win| {
            self.nvim.command(&format!("echom 'some win value={:?}'", win)).unwrap();
            ()
        }).collect();


        curwin
    }

    fn obsolete_put(&mut self, plist_str: Vec<String>) {
        // Before put was available :
        let win = self.nvim.get_current_win().unwrap();
        let (row, _col) = win.get_cursor(&mut self.nvim).unwrap();
        let buf = self.nvim.get_current_buf().unwrap();
        buf.set_lines(&mut self.nvim, row, row, true, plist_str).unwrap();
    }
}
*/
