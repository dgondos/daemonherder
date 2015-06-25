use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn read_to_string(path: &String) -> Result<String> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    return Ok(s);
}