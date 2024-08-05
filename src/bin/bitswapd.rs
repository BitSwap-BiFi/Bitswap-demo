use std::{Env, Process}
use std::env::Args;
pub struct Bitswapd;
pub struct Cli;
    
impl Bitswapd {
    fn start(args: Vec<String>) {
        println!("Starting bitswapd with args: {:?}", args);
    }
}
fn main() {
    let bin: Vec<String> = bitswapd::bin().collect();
    Bitswapd::start(args);
}
