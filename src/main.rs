#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

use std::path::{Path, PathBuf};
use std::collections::HashMap;

use rocket::response::{Failure, NamedFile};
use rocket::http::Status;
use rocket::request::Request;
use rocket_contrib::Template;

#[error(404)]
fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}

#[get("/")]
fn index() -> Template {
    Template::render("index", {})
}

#[get("/favicon.ico")]
pub fn icon() -> Failure {
    Failure(Status::NotFound)
}

#[get("/<path..>", rank=5)]
fn get_static(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(path)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![get_static, index, icon])
        .attach(Template::fairing())
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}
