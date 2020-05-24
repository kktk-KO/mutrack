use std::fs::File;
use std::io::prelude::*;
use std::string::String;

pub fn print_debug(text: String) {
    let mut file = File::create("/dev/stdout").unwrap_or_else(|_| panic!("Fialed to open /dev/stdout"));
    file.write_all(text.to_string().as_bytes()).unwrap_or_else(|_| panic!("Failed to write /dev/stdout"));
}
