use std::io::prelude::*;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Couldn't read from stdin");
    println!("Read: {:?}", input);
}
