pub struct Bitswapd;

impl Bitswapd {
    pub fn start(args: Vec<String>) {
        println!("Starting bitswapd with args: {:?}", args);
    }
}

fn main() {
    let bitswapd: Vec<String> = bitswapd::bitswapd().collect();
    Bitswapd::start(args);
}
