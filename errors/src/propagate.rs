use std::fs::{self, File};
use std::io::{self, Read};

const PATH_V1: &str = "../tmp/username_v1.txt";
const PATH_V2: &str = "../tmp/username_v2.txt";
const PATH_V3: &str = "../tmp/username_v3.txt";



pub fn read_username_from_file_v1() -> Result<String, io::Error> {
    let result = File::open(PATH_V1);

    let mut file = match result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

pub fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open(PATH_V2)?.read_to_string(&mut username)?;

    Ok(username)
}

pub fn read_username_from_file_v3() -> Result<String, io::Error> {
    return fs::read_to_string(PATH_V3);
}

pub fn username_last_char_of_file_line() -> Option<char> {
    fs::read_to_string(PATH_V2)
        .ok()?
        .lines()
        .next()?
        .chars()
        .last()
}