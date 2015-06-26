use std::fs;
use std::env;
use std::process::Command;

use config::Config;
use config::OptionalConfig;

pub struct ConfigHandler {
    next_path: fs::ReadDir,
    config_dir: String,
    hostname: String,
}

impl Iterator for ConfigHandler {
    type Item = OptionalConfig;

    fn next(&mut self) -> Option<OptionalConfig> {
        match self.next_path.next() {
            None => None,
            Some(path) => {
                let pathstring = path.unwrap().path().to_str().unwrap().to_string();
                let relpath = pathstring.replace(&self.config_dir, "");
                let relpath_tokens: Vec<_> = relpath.split('@').collect();

                if !pathstring.starts_with(&self.config_dir) {
                    Some(OptionalConfig::Invalid)
                }
                else if !pathstring.ends_with(".conf") {
                    Some(OptionalConfig::Invalid)
                }
                else if relpath_tokens.len() > 1 {
                    let nominated_hostname = relpath_tokens[relpath_tokens.len() - 1].trim_right_matches(".conf");
                    if nominated_hostname != self.hostname {
                        Some(OptionalConfig::Invalid)
                    }
                    else
                    {
                        Some(Config::from_json(&pathstring))
                    }
                }
                else {
                    Some(Config::from_json(&pathstring))
                }
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

// this is terrible, I know, but Rust doesn't seem to have any native way to
// retrieve the current hostname, and I prefer this to FFI'ing to
// libc's gethostname() (at least this is in pure Rust)
fn get_hostname() -> String {
    match Command::new("hostname").output() {
        Err(_) => String::new(), //not great either
        Ok(out) => {
            let hostname = String::from_utf8_lossy(&out.stdout).into_owned();
            hostname.trim().to_string()
        }
    }
}

pub fn configs() -> ConfigHandler {
    let config_dir = get_home() + "/.dherder/";

    ConfigHandler { next_path: get_configs(&config_dir),
                    config_dir: config_dir,
                    hostname: get_hostname() }
}