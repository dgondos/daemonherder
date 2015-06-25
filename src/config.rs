use util::read_to_string;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    pub name: String,
    pub cwd: String,
    pub cmd: String,
    pub args: Vec<String>
}

pub enum OptionalConfig {
    Valid(Config),
    Invalid
}

impl Config {
    pub fn from_json(path: &String) -> OptionalConfig {
        match read_to_string(path) {
            Err(e) => {
                println!("Could not read file {:?} due to {:?}", path, e.kind());
                OptionalConfig::Invalid
            },
            Ok(content_string) => {
                match json::decode(&content_string) {
                    Err(e) => {
                        println!("Could not parse config file: {:?}, due to: {:?}", path, e);
                        OptionalConfig::Invalid
                    },
                    Ok(new_config) => OptionalConfig::Valid(new_config)
                }
            }
        }
    }
}