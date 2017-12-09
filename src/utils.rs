use std::io::prelude::*;
use std::fs::File;

pub fn read_input(path: &str) -> String {
    let mut f = File::open(path).expect("File not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "Error reading the file contents.",
    );
    contents
}
