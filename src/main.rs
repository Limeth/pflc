#[macro_use]
extern crate nom;
#[macro_use]
extern crate clap;
// extern crate syntax;

use clap::Arg;

pub mod ast;
pub mod parser;

const ARG_SOURCE: &str = "source";

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name(ARG_SOURCE)
             .help("Path to the source file")
             .takes_value(true)
             .required(true))
        .get_matches();

    println!("Compiling '{}'...", matches.value_of(ARG_SOURCE).unwrap());
}
