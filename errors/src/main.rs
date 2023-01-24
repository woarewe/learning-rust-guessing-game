use std::fs::File;
use std::io::ErrorKind;


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
}
