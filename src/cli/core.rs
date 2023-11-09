use std::rgb_core;
use std::lightning;
use std::dlc;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("DEX CLI")
        .version("1.0.13-alpha")
        .author("Bitswap")
        .about("CLI for interacting with DEX on Bitcoin/RGB and Lightning Network")
        .subcommand(
            SubCommand::with_name("channel")
                .about("Manage Lightning Network channels")
                .subcommand(SubCommand::with_name("open").about("Open a new channel"))
                .subcommand(SubCommand::with_name("close").about("Close an existing channel"))
                .subcommand(SubCommand::with_name("rebalance").about("Rebalance a channel")),
        )
        .subcommand(SubCommand::with_name("contract").about("Interact with contracts on RGB"))
        .subcommand(SubCommand::with_name("wallet").about("Manage your wallet"))
        .subcommand(SubCommand::with_name("swap").about("Perform swaps"))
        .subcommand(SubCommand::with_name("mint").about("Mint new tokens"))
        .subcommand(SubCommand::with_name("oracle").about("See Oracle prices"))
        .get_matches();

    match matches.subcommand() {
        ("channel", Some(channel_matches)) => match channel_matches.subcommand() {
            ("open", Some(_)) => {
                // Logic to open a Lightning Network channel
                println!("Opening a new channel...");
               let mut input = String::new();
               let input = asset::BTC:();
               let input = asset::RGB_ASSET();
                  
            }
            ("close", Some(_)) => {
                // Logic to close a Lightning Network channel
                let mut output = String::new();
                let output = asset::BTC:();
                let output = asset::RGB_ASSET();
                println!("Closing a channel...");
            }
            ("rebalance", Some(_)) => {
                // Logic to rebalance a Lightning Network channel
                println!("Rebalancing a channel...");
            }
            _ => unreachable!(),
        },
        ("contract", Some(_)) => {
            // Logic to interact with RGB Core contracts
            println!("Interacting with contracts...");
        }
        ("wallet", Some(_)) => {
            // Logic to manage wallet
            println!("Managing wallet...");
        }
        ("swap", Some(_)) => {
            // Logic to perform swaps
            println!("Performing swaps...");
        }
        ("mint", Some(_)) => {
            // Logic to mint new tokens
            println!("Minting new tokens...");
        }
        _ => unreachable!(),
    }
}
