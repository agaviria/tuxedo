extern crate rustc_serialize;
extern crate toml_config;

use std::path::Path;
use toml_config::ConfigFactory;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    pub mysql: MySQLconf,
}

impl Config {
    pub fn mysql_uri(&self) -> String {
        self.mysql
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct MySQLconf {
    name: String,
    passwd: String,
    ip: String,
    port: String,
}

impl MySQLconf {
    pub fn uri(&self) -> String {
        let prefix = "mysql://".to_string();
        let my_name = self.name.to_string();
        let my_passwd = self.passwd.to_string();
        let my_ip = self.ip.to_string();
        let my_port = self.ip.to_string();
        let path = prefix + &uname + ":" + &my_passwd +  "@" &my_ip + ":" + &my_port;
        return path;
    }
}

impl Default for MySQLconf {
    fn default() -> MySQLconf {
        MySQLconf {
            uname: "root",
            passwd: "password",
            ip: "127.0.0.1".to_owned(),
            port: "3306".to_owned(),
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            mysql: MySQLconf::default(),
        }
    }
}

fn load() -> Config {
    let conf: Config = ConfigFactory::load(Path::new("conf.toml"))
}
