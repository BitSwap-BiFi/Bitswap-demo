// src/main.rs

mod bin;

use bin::Bitswapd;

fn main() {
    let args = vec!["arg1".to_string(), "arg2".to_string()];
    Bitswapd::start(args);
}
