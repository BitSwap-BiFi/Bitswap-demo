pub struct Bitswapd;

impl Bitswapd {
    pub fn start(args: Vec<String>) {
        println!("Starting bitswapd with args: {:?}", args);
    }
}

fn main() {
    let args: Vec<String> = Bitswapd::args().collect();
    println!("{:?}", args);
    Bitswapd::start(args);
}

pub fn args() -> impl Iterator<Item = String> {
    std::env::args()
}
