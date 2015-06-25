use std::thread;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::process::Command;

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
            loop {
                println!("Launching {:?} using command: {:?}, with args: {:?}, from directory: {:?}", config.name, config.cmd, config.args, config.cwd);
                let mut child = Command::new(&config.cmd)
                            .current_dir(&config.cwd)
                            .spawn()
                            .unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });

                child.wait().unwrap_or_else(|e| { panic!("failed to wait on child: {}", e) });
            }
        })
    }
}