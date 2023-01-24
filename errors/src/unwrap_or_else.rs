use std::fs::File;
use std::io::{ErrorKind, Error};

const PATH: &str = "../tmp/file.txt";

fn handle_opening(error: Error) -> File {
    if error.kind() == ErrorKind::NotFound {
        File::create(PATH).unwrap_or_else(handle_creating)
    } else {
        panic!("Problem opening the file: {:?}", error);
    }
}

fn handle_creating(error: Error) -> File {
    panic!("Creating file error: {:?}", error);
}

pub fn run() {
    let file = File::open(PATH).unwrap_or_else(handle_opening);
}