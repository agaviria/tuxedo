extern crate toml;
extern crate rustc_serialize;

use self::toml::Value;
use std::path::Path;
use std::io::Read;
use std::fs::File;
use rustc_serialize::Decodable;

pub struct Conf {
    pub toml: Value,
}

impl Conf {
    // Load a configuration file
    pub fn load_file(name: &str) -> Conf {
        let mut data = String::new();
        let mut file = File::open(Path::new(name)).ok().expect("config file cannot load");
        file.read_to_string(&mut data);

        // create Parser
        let mut conf_parser = toml::Parser::new(data.as_ref());

        let table = conf_parser.parse().unwrap();
        Conf { toml: Value::Table(table) }
    }

    // Access a value on the config file
    //
    // The key used is a period separated path such as "mysql.host"
    pub fn get_value<T: Decodable>(&mut self, key: &str) -> Option<T> {
        match self.toml.lookup(key) {
            Some(value) => toml::decode::<T>(value.clone()),
            None => None,
        }
    }
}
