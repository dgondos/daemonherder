use std::thread;
use std::sync::Arc;
use std::thread::JoinHandle;

use config::Config;

pub struct ProcessHandler {
    pub config: Config,
}

impl ProcessHandler {
    pub fn new(config: Config) -> ProcessHandler {
        ProcessHandler {config: config}
    }

    pub fn execute(self) -> JoinHandle<()> {
        let config = Arc::new(self.config).clone();
        thread::spawn(move|| {
            println!("Launching {:?} using command: {:?}, with args: {:?}, from directory: {:?}", config.name, config.cmd, config.args, config.cwd);
        })
    }
}