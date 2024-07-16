pub struct Bitswapd;
pub struct cli
    
impl Bitswapd {
    pub fn start(args: Vec<String>) {
        println!("Starting bitswapd with args: {:?}", args);
    }
}

fn main() {
    let bin: Vec<String> = bitswapd::bin().collect();
    Bitswapd::start(args);
}
