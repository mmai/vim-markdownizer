use async_trait::async_trait;
use tokio::io::Stdout;
use rmpv::Value;
use std::{error::Error, sync::Arc};
use futures::lock::Mutex;

use pathdiff::diff_paths;
use markdownizer::{types, Markdownizer};
use std::path::PathBuf;

use nvim_rs::{
  compat::tokio::Compat, Handler, Neovim, Window, error::CallError
};


type StoredProject = types::Stored<types::Project>;

// #[derive(Clone)]
pub struct State {
    loaded: bool,
    // content_win: Option<Value>,
    content_win: Option<Window<Compat<Stdout>>>,
    projects: Vec<StoredProject>
}

impl Default for State {
    fn default() -> Self {
        State {
            loaded: false,
            projects: vec![],
            content_win: None
        }
    }
}

#[derive(Clone)]
pub struct NeovimHandler{
    state: Arc<Mutex<State>>,
    markdownizer: Markdownizer,
}

impl NeovimHandler {
    pub fn new(projects_dir: &str, state: Arc<Mutex<State>>) -> Self {
        NeovimHandler {
            // state: State { loaded: false, projects: vec![] },
            state,
            markdownizer: Markdownizer::new(&projects_dir.into())
        }
    }

    async fn get_project_list(&self, nvim: &Neovim<Compat<Stdout>>) -> Result<Vec<String>, markdownizer::MarkdownizerError> {
        // let curr_dir: PathBuf = self.nvim.command_output("echo expand('%:p:h')").unwrap().into();
        // let curr_dir: PathBuf = self.vim_ask("expand", vec!("%:p:h")).unwrap().into();
        let curr_dir: PathBuf = self.vim_ask(&nvim, "expand('%:p:h')").await.unwrap().into();
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

    async fn init_state(&self) {
        let result = self.markdownizer.project_list();

        let mut state = &mut *(self.state).lock().await;
        state.projects = result.unwrap_or(vec!());
    }

    // Call a vim function which return output
    // fn vim_ask(&mut self, func: &str, params: Vec<&str>) -> Result<String, neovim_lib::neovim::CallError> {
        // self.nvim.call_function(func, params.into_iter().map(|v| v.into()).collect())
    async fn vim_ask(&self, nvim: &Neovim<Compat<Stdout>>, expr: &str) -> Result<String, Box<CallError>> {
        nvim.eval(expr).await.map(|val| String::from( val.as_str().unwrap() ))
    }

}

#[async_trait]
impl Handler for NeovimHandler {
  type Writer = Compat<Stdout>;

  // responds to 'rpcrequest' calls from nvim plugin
  async fn handle_request(
    &self,
    name: String,
    _args: Vec<Value>,
    neovim: Neovim<Compat<Stdout>>,
  ) -> Result<Value, Value> {
    match name.as_ref() {
      "init_content_window" => {
          neovim.command("echom 'init_content_window ..'").await.unwrap();
        Ok(Value::Nil)
      },
      _ => Ok(Value::Nil)
    }
  }


  // responds to 'rpcnotify' calls from nvim plugin
  async fn handle_notify(
      &self,
      name: String,
      mut _args: Vec<Value>,
      nvim: Neovim<Compat<Stdout>>,
  ) {
      match name.as_ref() {
          "init_content_window" => {
              let content_win = nvim.get_current_win().await.unwrap();
              // self.state.content_win = Some(*content_win);
              nvim.command(&format!("echom 'initwin value={:?}'", content_win.get_value())).await.unwrap();

              // let wins = nvim.list_wins().await.unwrap();
              // let _:() = wins.iter().map(|win| {
                  // nvim.command(&format!("echom 'some win value={:?}'", win)).unwrap();
                  // ()
              // }).collect();
          }
          "dashboard" => {
              // nvim.command(&format!("echom 'in dashboard rs'")).await.unwrap();
              // let buf = values.pop().unwrap().into();
              let buf = nvim.get_current_buf().await.unwrap();
              let state = (self.state).lock().await;
              if (! &state.loaded){
                  self.init_state().await;
              }
              let plist = state.projects.iter().map(|p| String::from(&p.entity.title)).collect();
              buf.set_lines(0, -1, true, plist).await.unwrap();
          },
          "project_list" => {
              let result = self.get_project_list(&nvim).await;
              match result {
                  Ok(plist) => {
                      nvim.put(plist, "", true, true).await.unwrap();
                  },
                  Err(e) => {
                      nvim.err_writeln(&format!("Error when reading projects : {}", e)).await.unwrap();
                  }
              }
          }
          "project_select" => {
              let line = _args.pop().unwrap().as_i64().unwrap();
              let win_id = _args.pop().unwrap();

              let curr_dir: PathBuf = self.vim_ask(&nvim, "expand('%:p:h')").await.unwrap().into();
              let state = (self.state).lock().await;
              let stored_project:&StoredProject = &state.projects[line as usize - 1];
              // let stored_project:&StoredProject = self.state.projects.get(line.into()).unwrap();
              let project = &stored_project.entity;
              let location = &stored_project.location;
              let relative_path = diff_paths(location.as_path(), &curr_dir).unwrap();
              let file_path = String::from(format!("{}", relative_path.to_str().unwrap()));

              // let cwin = Window {
              //     code_data: self.state.content_win.clone().unwrap(),
              //     neovim: nvim,
              // };
              // nvim.set_current_win(&cwin).await.unwrap();
              // self.nvim.command(&format!("echo '{}'", file_path)).unwrap();
              nvim.command(&format!("e {}", file_path)).await.unwrap();
              // self.nvim.err_writeln(&format!("{}", file_path)).unwrap();
          }
          _ => {}

      }
  }

}
