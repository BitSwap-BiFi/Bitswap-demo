use bdk::bitcoin::Network;
use bdk::blockchain::{ElectrumBlockchain, Blockchain};
use bdk::database::MemoryDatabase;
use bdk::wallet::{Wallet, SyncOptions};
use bdk::electrum::electrum_client::Client;
use std::process::Command;

fn main() {
    // Set up the Electrum server (testnet in this example)
    let electrum_url = "ssl://electrum.blockstream.info:60002";

    // Set up the database and wallet configuration
    let database = MemoryDatabase::new();
    let wallet = Wallet::new("wpkh([84'/1'/0'/0]tpubD6NzVbkrYhZ4YM4Wsor1cQtv3kJvMeLFx1MRz7Jc81t4UPR2xNnt3oLhRLtfTBLGmQEdk1nVJpr5YTGscdLQR1Jz8yHoDeQ8RyYAGDzFEXD/*)", None, Network::Testnet, database).unwrap();

    // Set up the blockchain client
    let client = Client::new(electrum_url).unwrap();
    let blockchain = ElectrumBlockchain::from(client);

    // Sync the wallet
    wallet.sync(&blockchain, SyncOptions::default()).unwrap();

    // Print wallet balance
    let balance = wallet.get_balance().unwrap();
    println!("Wallet balance: {} sats", balance);

    // Print a new receiving address
    let address = wallet.get_address(bdk::wallet::AddressIndex::New).unwrap();
    println!("New address: {}", address);

    // Example of calling RGB CLI to list RGB assets
    let output = Command::new("rgb-cli")
        .arg("assets")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let assets = String::from_utf8_lossy(&output.stdout);
        println!("RGB Assets:\n{}", assets);
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error listing RGB assets: {}", err);
    }
}
