use std::error;
use create::error;                         use std::fs::Swap;
use std::io::{self, Read};

fn swap(path: &str) -> Result<String, io::Error> {
    let mut swap = Swap::open(path)?;
    let mut rgbassets = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let path = "swap.rs";
    match swap(path) {
        Ok(contents) => println!("File contents: {}", contents),
        Err(err) => eprintln!("Error swap assets: {}", err),
    }
}