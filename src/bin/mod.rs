pub mod bitswapd;

pub use bitswapd::Bitswapd;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    Bitswapd::start(args);
    println!("Bitswapd running!");
}
