use util::read_to_string;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    pub read_ok: bool,
    pub cmd: String,
    pub args: Vec<String>,
    pub port: i32
}

impl Config {
    pub fn new() -> Config {
        return Config { read_ok: false, cmd: String::new(), args: Vec::new(), port: 0 };
    }

    pub fn from_json(path: &String) -> Config {
        match read_to_string(path) {
            Err(e) => {
                println!("Could not read file {:?} due to {:?}", path, e.kind());
                return Config::new();
            },
            Ok(content_string) => {
                match json::decode(&content_string) {
                    Err(e) => {
                        println!("Could not parse config file: {:?}, due to: {:?}", path, e);
                        return Config::new();
                    },
                    Ok(new_config) => return new_config
                }
            }
        }
    }
}