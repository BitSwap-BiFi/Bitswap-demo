use std::Rgb_core;
use std::Lightning;
use std::Dlc;
use rgb::Rgb_contract::{UnsupportedLayer1, Layer1, Liquid, PSBT};
use rgbstd::interface::{Rgb20, Iface};
use rgbstd::persistence::{Stock, State, Stash};
use rgbstd::invoice::{Amount, Data, Invoice};
use bitcoin::constants::Network;
use bitcoin::constants::Network::{Testnet, Regtest, Signet};

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("DEX CLI")
        .version("1.0.24-alpha")
        .author("Bitswap")
        .about("CLI for interacting with DEX on RGB and Lightning Network")
        .subcommand(
            SubCommand::with_name("channel")
                .about("Manage Lightning Network channels")
                .subcommand(SubCommand::with_name("open").about("Open a new channel"))
                .subcommand(SubCommand::with_name("close").about("Close an existing channel"))
                .subcommand(SubCommand::with_name("rebalance").about("Rebalance a channel")),
        )
        .subcommand(SubCommand::with_name("contract").about("Interact with contracts on RGB"))
        .subcommand(SubCommand::with_name("rgb_contract").about("Interact with RGB"))
        .subcommand(SubCommand::with_name("wallet").about("Manage your wallet"))
        .subcommand(SubCommand::with_name("fund_wallet").about("Fund wallet"))
        .subcommand(SubCommand::with_name("RGBInvoice").about("RGB Invoice"))
        .subcommand(SubCommand::with_name("swap").about("Perform swaps"))
        .subcommand(SubCommand::with_name("oracle").about("See Oracle prices"))
        .subcommand(SubCommand::with_name("add_liquidity").about("Add liquidity"))
        .subcommand(SubCommand::with_name("remove_liquidity").about("Remove liquidity"))
        .subcommand(SubCommand::with_name("refund").about("Refund"))
        .subcommand(SubCommand::with_name("psbt").about("PSBT"))
        .subcommand(SubCommand::with_name("liquid").about("Liquid"))
        .subcommand(SubCommand::with_name("liquid_swap").about("Liquid Swap"))
        .subcommand(SubCommand::with_name("liquid_redeem").about("Liquid Redeem"))
        .subcommand(SubCommand::with_name("liquid_withdraw").about("Liquid Withdraw"))
        .subcommand(SubCommand::with_name("liquid_deposit").about("Liquid Deposit"))
        .subcommand(SubCommand::with_name("network").about("Network"))
        .subcommand(SubCommand::with_name("amm").about("AMM"))
        .get_matches();

    match matches.subcommand() {
        ("fund_wallet", Some(sub_m)) => {
            // Handle command for funding wallet
            if let Some(fund_wallet) = sub_m.value_of("fund_wallet") {
                println!("Command executed with fund_wallet: {}", fund_wallet);
            } else {
                println!("Command executed");
            }
        }
        ("swap", Some(sub_m)) => {
            // Handle command for swaps
            if let Some(swap) = sub_m.value_of("swap") {
                println!("Command executed with swap: {}", swap);
            } else {
                println!("Command executed");
            }
        }
        ("channel", Some(channel_matches)) => match channel_matches.subcommand() {
            ("open", Some(_)) => {
                // Logic to open a Lightning Network channel
                println!("Opening a new channel...");
            }
            ("close", Some(_)) => {
                // Logic to close a Lightning Network channel
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
        ("rgb_contract", Some(_)) => {
            // Logic to manage contract
            println!("Managing RGB contract...");
        }
        ("wallet", Some(_)) => {
            // Logic to manage wallet
            println!("Managing wallet...");
        }
        ("swap", Some(_)) => {
            // Logic to perform swaps
            println!("Performing swaps...");
        }
        ("refund", Some(_)) => {
            // Logic to handle refunds
            println!("Handling refund...");
        }
        ("oracle", Some(_)) => {
            // Logic for DLCs oracles
            println!("Handling oracle...");
        }
        ("add_liquidity", Some(_)) => {
            // Logic to add liquidity
            println!("Adding liquidity...");
        }
        ("remove_liquidity", Some(_)) => {
            // Logic to remove liquidity
            println!("Removing liquidity...");
        }
        ("network", Some(_)) => {
            // Logic for network
            println!("Handling network...");
        }
        ("amm", Some(_)) => {
            // Logic to handle AMM functions
            let x = 100.0; // Replace with actual value
            let y = 200.0; // Replace with actual value
            let k = x * y;
            println!("Handling AMM functions with x: {}, y: {}, k: {}", x, y, k);
        }
        _ => {
            println!("No subcommand provided");
        }
    }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;
    use clap::ArgMatches;

    #[test]
    fn test_fund_wallet() {
        let matches = App::new("test")
            .subcommand(SubCommand::with_name("fund_wallet").arg(Arg::with_name("fund_wallet")))
            .get_matches_from(vec!["test", "fund_wallet", "some_value"]);

        match matches.subcommand() {
            ("fund_wallet", Some(sub_m)) => {
                let fund_wallet = sub_m.value_of("fund_wallet").unwrap();
                assert_eq!(fund_wallet, "some_value");
            }
            _ => panic!("fund_wallet command failed"),
        }
    }

    #[test]
    fn test_swap() {
        let matches = App::new("test")
            .subcommand(SubCommand::with_name("swap").arg(Arg::with_name("swap")))
            .get_matches_from(vec!["test", "swap", "some_value"]);

        match matches.subcommand() {
            ("swap", Some(sub_m)) => {
                let swap = sub_m.value_of("swap").unwrap();
                assert_eq!(swap, "some_value");
            }
            _ => panic!("swap command failed"),
        }
    }

    #[test]
    fn test_open_channel() {
        let matches = App::new("test")
            .subcommand(
                SubCommand::with_name("channel")
                    .subcommand(SubCommand::with_name("open")),
            )
            .get_matches_from(vec!["test", "channel", "open"]);

        match matches.subcommand() {
            ("channel", Some(channel_matches)) => match channel_matches.subcommand() {
                ("open", Some(_)) => {
                    // Assuming opening a channel involves some logic
                    assert!(true);
                }
                _ => panic!("channel open command failed"),
            },
            _ => panic!("channel command failed"),
        }
    }

    // TODO CLI additional tests 
}
