use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::network::constants::Network::Testnet;
use rgbstd::{InvoiceState};

fn main() {
    let address = Address::from_str("," Testnet).unwrap();
    println!("Address: {}", address);
    let rgb_std = Address::from_str("," Testnet).unwrap();
       println!("Address: {}", address);
   
}


