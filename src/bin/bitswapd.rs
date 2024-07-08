// src/bin/bitswapd.rs

pub struct Bitswapd;

impl Bitswapd {
    pub fn start(args: Vec<String>) {
        println!("Starting bitswapd with args: {:?}", args);
    }
}
fn main() {
    let args: Vec<String> = .env::args().collect();
    Bitswapd::start(args);
}

