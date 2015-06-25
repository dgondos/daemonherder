use std::fs;
use std::env;

use config::Config;
use config::OptionalConfig;

pub struct ConfigHandler {
    next_path: fs::ReadDir,
}

impl Iterator for ConfigHandler {
    type Item = OptionalConfig;

    fn next(&mut self) -> Option<OptionalConfig> {
        match self.next_path.next() {
            None => None,
            Some(path) => {
                let pathstring = path.unwrap().path().to_str().unwrap().to_string();
                Some(Config::from_json(&pathstring))
            }
        }
    }
}

fn create_home(path: &String) {
    fs::create_dir(path).unwrap();
}

fn get_home() -> String {
    let key = "HOME";
    match env::var(key) {
        Ok(val) => val,
        Err(_) => ".".to_string(),
    }
}

fn get_configs(config_dir: &String) -> fs::ReadDir {
    match fs::read_dir(&config_dir) {
        Err(_) => { create_home(&config_dir); get_configs(&config_dir) },
        Ok(paths) => return paths
    }
}

pub fn configs() -> ConfigHandler {
    let config_dir = get_home() + "/.dherder";

    ConfigHandler { next_path: get_configs(&config_dir) }
}