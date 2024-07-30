pub struct Bitswapd;
pub struct Cli;
    
impl Bitswapd for Bitswapd {
    fn start(args: Vec<String>) {
        println!("Starting bitswapd with args: {:?}", args);
    }
}

fn main() {
    let bin: Vec<String> = bitswapd::bin().collect();
    Bitswapd::start(args);
}
