#[macro_use]
extern crate nom;
#[macro_use]
extern crate clap;
// extern crate syntax;

use std::io::{self, Read};
use std::fs::File;
use clap::Arg;

pub mod ast;
pub mod parser;

const ARG_SOURCE: &str = "source";

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut result = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut result)?;
    Ok(result)
}

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name(ARG_SOURCE)
             .help("Path to the source file")
             .takes_value(true)
             .required(true))
        .get_matches();

    let source = read_file(matches.value_of(ARG_SOURCE).unwrap()).unwrap_or_else(|error| {
        eprintln!("Could not read the source file.");
        std::process::exit(1);
    });

    println!("Compiling '{}'...", matches.value_of(ARG_SOURCE).unwrap());
    println!("{:?}", parser::parse(source));
}
