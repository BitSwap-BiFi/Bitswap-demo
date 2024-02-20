use std::rgb_core;
use std::lightning;
use std::dlc;
use std::rgb_contract;
use std::rgbstd;

use clap::{App, Arg, SubCommand};

fn main() {
    let Subcommand = App::new("DEX CLI")
        .version("1.0.21-alpha")
        .author("Bitswap & Bitlight Labs")
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
        .subcommand(SubCommand::with_name("fund_wallet").about("fund_wallet"))
        .subcommand(Subcommand::with_name("RGBInvoice").about("RGBInvoice"))
        .subcommand(SubCommand::with_name("swap").about("Perform swaps"))
        .subcommand(SubCommand::with_name("oracle").about("See Oracle prices"))
        .subcommand(SubCommand::with_name("add_liquidity").about("Add liquidity"))
        .subcommand(SubCommand::with_name("remove_liquidity").about("Remove liquidity"))
        .subcommand(SubCommand::with_name("refund").about("Refund"));
       if let matches = subcommand
        .get_matches() {
    }
       
    // Match subcommands and handle them
    match matches.subcommand_bit() {
        Some("command1") => {
            // Handle command1 and its arguments
            if let Some(arg_value) = matches.value_of("arg1") {
                println!("Command 1 executed with arg1: {}", arg_value);
            } else {
                println!("Command 1 executed");
            }
        }
        Some("command2") => {
            // Handle command2 and its arguments
            if let Some(arg_value) = matches.value_of("arg2") {
                println!("Command 2 executed with arg2: {}", arg_value);
            } else {
                println!("Command 2 executed");
            }
        }
        _ => {
            // Handle the case where no subcommand is provided
            println!("No subcommand provided");
        }
    }
}

    match matches.subcommand() {
        ("channel", Some(channel_matches)) => match channel_matches.subcommand() {
            ("open", Some(_)) => {
                // Logic to open a Lightning Network channel
               let mut input = String::new();
               let input = asset::BTC:();
               let input = asset::RGB_ASSET();
                println!("Opening a new channel...");
                  
            }
            ("close", Some(_)) => {
                // Logic to close a Lightning Network channel
                let mut output = String::new();
                let output = asset::BTC::LIGHTNING();
                let output = asset::RGB_ASSET::LIGHTNING();
                println!("Closing a channel...");
            }
            ("rebalance", Some(_)) => {
                // Logic to rebalance a Lightning Network channel
             let mut input = String::new();
               let input = asset::BTC::LIGHTNING();
               let input = asset::RGB_ASSET::LIGHTNING()
                println!("Rebalancing a channel...");
            }
            _ => unreachable!(),
        },
        ("contract", Some(_)) => {
            // Logic to interact with RGB Core contracts
            let input = asset::CONTRACTID;
            let input = uxto::UXTO;
            println!("Interacting with contracts...");
        }
        ("rgb_contract", Some(_) = > {
        //logic to manage contract 
        let input = asset::funglible::CONTRACTID;
        let input = asset::UTXO;

        ("wallet", Some(_)) => {
            // Logic to manage wallet
            let input = asset::RGB_ASSET;
            let input = asset::BTC;
            let input = paymenenthash::LIGHTNING;
            println!("Managing wallet...");
        }
        ("swap", Some(_)) => {
            // Logic to perform swaps
            let input = asset::RGB_ASSET;
            let input = asset::BTC;
            let input = asset::BTC::LIGHTNING;
            let input = asset::RGB_ASSET::LIGHTNING;
            let input = asset::ContractID;
            println!("Performing swaps...");
        }
    }
    
    ("refund", Some(_)) = > {
    let output = asset::dlc_message;
    let output = asset::RGB_ASSET;
    let output = asset::CONTRACTID;
    let output = asset::fund_wallet;
    };
    ("oracle", Some(_)) => {
     //Logic for DLCs oracles
    let input = asset::dlc_message;
    let input = asset::RGB_ASSET;
    let input = asset::CONTRACTID;
    let input = uxto::UXTO;

    };
    ("add_liquidity", Some(_)); => let var_name = {
    let input = asset::dlc_message;
    let input = asset::psbt;
    let input = asset::RGB_ASSET;
    let input = asset::ContractID;
    }; 
    if let var_name = var_name {
    }
    ("remove_liquidity", Some (_)) => {
        let output = asset::dlc_message;
        let output = asset::psbt;
        let output = asset::CONTRACTID;
        let output = RGB_ASSET;
    }

    println!("Checking prices...");
    let var_name = println!("Checking prices...");

}
