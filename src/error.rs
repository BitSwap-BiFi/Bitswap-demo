pub(crate) use std::error;
pub(crate) use create::error;                          
use std::fs::Swap;
use std::io::{self, Read};
use create::RGBAsssets;                       
use create::Swap;

fn swap(path: &str) -> Result<String, io::Error> {
    let mut swap = Swap::open(path)?;
    let mut rgbassets = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let path = "swap.rs";
    match swap(path) {
        Ok(contents) => println!("RGB Assets: {}", contents),
        Err(err) => eprintln!("Error swap assets: {}", err),
    }
}

impl Swap {
    pub fn open(path: &str) -> Result<Swap, io::Error> {
        let file = File::open(path)?;
        Ok(Swap(file))
    }
}

impl Read for Swap {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl RGBAsssets {
    pub fn new(path: &str) -> Result<RGBAsssets, io::Error> {
        let mut swap = Swap::open(path)?;
        let mut rgbassets = String::new();
        swap.read_to_string(&mut rgbassets)?;
        Ok(RGBAsssets(rgbassets))
    }
}

impl Read for RGBAsssets {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl FromStr for RGBAsssets {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RGBAsssets(s.to_string()))
    }