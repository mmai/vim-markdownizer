use std::env;

mod eventhandler;

use std::{error::Error, sync::Arc};
use futures::lock::Mutex;
use nvim_rs::create::tokio as create;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let projects_dir = &args[1]; // projects files directory

    let state = Arc::new(Mutex::new(eventhandler::State::default()));

    let handler: eventhandler::NeovimHandler = eventhandler::NeovimHandler::new(projects_dir, state);
    let (nvim, io_handler) = create::new_parent(handler).await;

    // Any error should probably be logged, as stderr is not visible to users.
    match io_handler.await {
        Err(joinerr) => eprintln!("Error joining IO loop: '{}'", joinerr),
        Ok(Err(err)) => {
            if !err.is_reader_error() {
                // One last try, since there wasn't an error with writing to the
                // stream
                nvim
                    .err_writeln(&format!("Error: '{}'", err))
                    .await
                    .unwrap_or_else(|e| {
                        // We could inspect this error to see what was happening, and
                        // maybe retry, but at this point it's probably best
                        // to assume the worst and print a friendly and
                        // supportive message to our users
                        eprintln!("Well, dang... '{}'", e);
                    });
            }

            if !err.is_channel_closed() {
                // Closed channel usually means neovim quit itself, or this plugin was
                // told to quit by closing the channel, so it's not always an error
                // condition.
                eprintln!("Error: '{}'", err);

                let mut source = err.source();

                while let Some(e) = source {
                    eprintln!("Caused by: '{}'", e);
                    source = e.source();
                }
            }
        }
        Ok(Ok(())) => {}
    }
}
