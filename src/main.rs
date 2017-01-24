#[macro_use]
extern crate clap;

mod cli;

fn main() {
    let options = cli::gen_cli().get_matches();
}
