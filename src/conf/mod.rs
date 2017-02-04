extern crate toml;
extern crate rustc_serialize;

use self::toml::Value;
use std::path::Path;
use std::io::Read;
use std::fs::File;
use rustc_serialize::Decodable;

static CONFIG_FILE: &'static str = "config.toml";

pub struct Config {
    pub toml: Value,
}

impl Config {
    // Load a configuration file
    pub fn load_file() -> Config {
        let mut content = String::new();
        let mut file = File::open(Path::new(CONFIG_FILE)).ok().expect("config file did not load");
        file.read_to_string(&mut content);

        // create Parser
        let mut cfg_parser = toml::Parser::new(content.as_ref());

        let value = cfg_parser.parse().unwrap();
        Config { toml: Value::Table(value) }
    }

    // Access a value on the config file
    // The key used is a period separated path such as "mysql.host"
    pub fn get_value<T: Decodable>(&mut self, key: &str) -> Option<T> {
        match self.toml.lookup(key) {
            Some(value) => toml::decode::<T>(value.clone()),
            None => None,
        }
    }
}
