use std::fs::File;
use std::io::prelude::*;
use std::process;

pub fn read_file(filename: &str) -> String {

    let mut file = File::open(filename).unwrap_or_else(|err| {
        eprintln!("Problem opening file. {:?}\n error: {}\n ", filename, err);
        process::exit(1)
    });

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap_or_else(|err| {
        eprintln!("Problem reading file.\n error: {}", err);
        process::exit(1)
    });
    contents
}