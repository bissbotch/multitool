use std::fs::File;
use std::io::prelude::*;

pub fn find_file() -> &String {
    let mut file = File::open("passwords.txt")
        .expect("Cannot open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot read the file");

    
    return contents;
}

pub fn read_file(contents) {
    println!("File contents: \n\n{}", contents);
}