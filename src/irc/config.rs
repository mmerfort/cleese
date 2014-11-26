// For copyright information, see the LICENSE.md folder at the top of this
// project's directory structure.

use regex::Regex;
use serialize::{json, Decodable};
use std::io::{File, Open, Read};

// Single server for now.
pub struct IrcConfig<'a> {
    pub host: &'a str,
    pub port: u16,
    pub nick: &'a str,
    pub descr: &'a str,
    pub channels: Vec<&'a str>,
    pub in_blacklist: Vec<&'a str>,
    pub out_blacklist: Vec<Regex>,
    pub cmd_prefix: &'a str,
}

#[deriving(Decodable)]
pub struct JsonConfig {
    pub host: String,
    pub port: u16,
    pub nick: String,
    pub descr: String,
    pub channels: Vec<String>,
    pub in_blacklist: Vec<String>,
    pub out_blacklist: Vec<String>,
    pub cmd_prefix: String
}

impl JsonConfig {
    pub fn new(location: String) -> JsonConfig {
        let p = Path::new(location.as_slice());
        let mut file = match File::open_mode(&p, Open, Read) {
            Ok(f) => f,
            Err(e) => panic!("file error: {}", e)
        };

        let decoded: String = match file.read_to_string() {
            Ok(f) => f,
            Err(e) => panic!("file error: {}", e)
        };

        let json_object = match json::from_str(decoded.as_slice()) {
            Ok(x) => x,
            Err(e) => panic!("json error: {}", e)
        };
        let mut decoder = json::Decoder::new(json_object);

        return match Decodable::decode(&mut decoder) {
            Ok(v) => v,
            Err(e) => panic!("Decoding error: {}", e)
        };
    }
}
