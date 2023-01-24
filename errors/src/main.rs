use std::fs::File;


fn main() {
    let result = File::open("../tmp/helsflo.txt");


    let file = match result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
