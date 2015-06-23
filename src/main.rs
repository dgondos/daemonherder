extern crate rustc_serialize;

mod config_handler;
mod util;

fn main() {
    for testvar in config_handler::configs() {
        println!("{:?}", testvar.cmd);
    }
}