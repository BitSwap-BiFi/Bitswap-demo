pub mod bitswapd;

pub use crate bitswapd::Bitswapd;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    Bitswapd::start(args);
    println!("Bitswapd running!");
}
