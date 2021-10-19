use serde_pickle::DeOptions;
use std::io::{self, Read};

fn main() {
    // read contents from stdin
    let mut contents = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut contents).ok();

    // deserialize the pickle dump
    let deserialized: Vec<Vec<(String, i64)>> =
        serde_pickle::from_slice(&contents.as_bytes(), DeOptions::default()).unwrap();

    // print the results
    for list in deserialized {
        for (bytes, num) in list {
            print!("{}", bytes.repeat(num as usize));
        }
        println!("");
    }
}
