extern crate rustc_serialize;

use std::thread::JoinHandle;

mod config;
use config::OptionalConfig;
mod config_handler;
mod process_handler;
use self::process_handler::ProcessHandler;
mod util;

fn main() {
    let mut handlers:Vec<JoinHandle<()>> = Vec::new();
    for optional_config in config_handler::configs() {
        match optional_config {
            OptionalConfig::Invalid => continue,
            OptionalConfig::Valid(config) => {
                let handler = ProcessHandler::new(config);
                let thread = handler.execute();
                handlers.push(thread);
            }
        }
    }

    for handler in handlers {
        handler.join().unwrap();
    }
}