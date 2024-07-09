use std::env;

struct Bitswapd;

impl Bitswapd {
    pub fn start(args: Vec<String>) {
        println!("Starting bitswapd with args: {:?}", args);
    }
}

fn main() {
    let args: Vec<String> = bitswapd::args().collect();
    // rest of the code
}

pub fn args() -> impl Iterator<Item = String> {
    std::env::args()
}