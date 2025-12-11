use std::fs;
use std::io::prelude::*;

// Useful resource for learning rust standard library: https://docs.rs/rustc-std-workspace-std/latest/std/

fn main() {
    // Create output.txt file
    fs::write("output.txt", "Hello, world!").unwrap();

    // Open output.txt and save to file variable, create string, read file to string
    let mut file = fs::File::open("output.txt").unwrap();
    let mut s = String::new(); //
    file.read_to_string(&mut s).unwrap();

    // Split string s at whitespace, print each word on new line with debug formatting
    s.split_whitespace().for_each(|word| println!("{:?}", word));
}
