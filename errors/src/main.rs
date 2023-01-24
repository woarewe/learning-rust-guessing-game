use std::fs::File;
use std::io::ErrorKind;

pub mod unwrap_or_else;
pub mod propagate;

fn main() {
    let path = "../tmp/helsflo.txt";
    let result = File::open(path);

    let file = match result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Creating file failed {:?}", e)
            },
            other_error => {
                panic!("Problem with opening file {:?}", other_error);
            }
        }
    };

    unwrap_or_else::run();

    println!("Result1: {:?}", propagate::read_username_from_file_v1());
    println!("Result2: {:?}", propagate::read_username_from_file_v2());
    println!("Result3: {:?}", propagate::read_username_from_file_v3());
    println!("Optional char {:?}", propagate::username_last_char_of_file_line());
}
