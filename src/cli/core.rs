use std::Rgb_core;
use std::Lightning;
use std::Dlc;
use std::rgb_contract::{UnsupportedLayer1, Layer1, Liquid, PSBT};
use std::rgbstd::interface::{Rgb20, Iface};
use std::rgbstd::persistence::{Stock, State, Stash};
use std::rgbstd::invoice::{Amount, Data, Invoice};
use bitcoin::constants::Network;
use bitcoin::constants::Network::Testnet;
use bitcoin::constants::Network::Regtest;
use bitcoin::constants::Network::Signet;


use clap::{App, Arg, SubCommand};
use crate::Core::{Cli, SubCommand,Subcommand_bit};
use crate::Core::{Contract, RGBContract, Refund, RemoveLiquidity, Oracle, FundWallet, PSBT, AddLiquidity, Swap, RGBInvoice, Wallet, Network, AMM, Layer1, Liquid, Liquid_deposit, Liquid_swap};


fn main() 
    let _subcommand = App::new("DEX CLI")
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
        .subcommand(SubCommand::with_name("fund_wallet").about("fund_wallet"))
        .subcommand(Subcommand::with_name("RGBInvoice").about("RGBInvoice"))
        .subcommand(SubCommand::with_name("swap").about("Perform swaps"))
        .subcommand(SubCommand::with_name("oracle").about("See Oracle prices"))
        .subcommand(SubCommand::with_name("add_liquidity").about("Add liquidity"))
        .subcommand(SubCommand::with_name("remove_liquidity").about("Remove liquidity"))
        .subcommand(SubCommand::with_name("refund").about("Refund"))
        .subcommand(SubCommand::with_name("psbt").about("PSBT"))
        .subcommand(subcommand::with_name("liquid").about("Liquid"))
        .subcommand(subcommand::with_name("liquid_swap").about("Liquid Swap"))
        .subcommand(subcommand::with_name("liquid_redeem").about("Liquid Redeem"))
        .subcommand(subcommand::with_name("liquid_withdraw").about("Liquid Withdraw"))
        .subcommand(subcommand::with_name("liquid_deposit").about("Liquid Deposit"))
        .subcommand(subcommand::with_name("network").about("Network"))
        .subcommand(subcommand::with_name("amm").about("AMM"))
        .get_matches();
   
        match matches.subcommand_bit{
        Some("fund_wallet") => {
            // Handle command1 and its arguments
            if let Some(fund_wallet) = matches.value_of("fund_wallet") {
                println!("Command 1 executed with fund_wwallet: {}", fund_wallet);
            } else {
                println!("Command 1 executed");
            }
        }
        }
        Some("swap") => {
            // Handle command2 and its arguments
            if let Some(swap) = matches.value_of("swap") {
                println!("Command 2 executed with swap: {}", swap);
            } else {
                println!("Command 2 executed");
            }
        }
        _ => {
                // Handle the case where no subcommand is provided
                    if let subcommand_bit(swap, fund_wallet).let value_of = error.value_of("swap, fund_wallet");
                    println!("No subcommand provided");
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
        match matches.subcommand() 
      ("rgb_contract", Some(_)) => {
        //logic to manage contract 
        let input = asset::funglible::CONTRACTID;
        let input = asset::UTXO;
    }
        ("wallet", Some(_)) => {
            // Logic to manage wallet
            let input = asset::RGB_ASSET;
            let input = asset::BTC;
            let input = paymenenthash::LIGHTNING;
            let input = asset::psbt;
            println!("Managing wallet...");
        }
        ("swap", Some(_)) => {
            // Logic to perform swaps
            let input = asset::RGB_ASSET;
            let input = asset::BTC;
            let input = asset::BTC::LIGHTNING;
            let input = asset::RGB_ASSET::LIGHTNING;
            let input = asset::ContractID;
            let input = asset::psbt;
            println!("Performing swaps...");
        }
    }
    
    ("refund", Some(_)) = > {
    let output = asset::dlc_message;
    let output = asset::RGB_ASSET;
    let output = asset::CONTRACTID;
    let output = asset::fund_wallet;
    let output = asset::psbt;
    };
    ("oracle", Some(_)) => {
     //Logic for DLCs oracles
    let input = asset::dlc_message;
    let input = asset::RGB_ASSET;
    let input = asset::CONTRACTID;
    let input = uxto::UXTO;
    let input = asset::psbt;

    };
    ("add_liquidity", Some(_)) = {
    let input = asset::dlc_message;
    let input = asset::psbt;
    let input = asset::RGB_ASSET;
    let input = asset::ContractID;
    let input = asset::psbt;
    }; 
    if let var_name = var_name {
    }
    ("remove_liquidity", Some (_)) => {
        let output = asset::dlc_message;
        let output = asset::psbt;
        let output = asset::CONTRACTID;
        let output = asset::RGB_ASSET;
        let output = asset::psbt;
    }
    ("network", Some(_) = > {
        let network = Testnet;
        let network = Signet;

    )}
    ("amm", Some(_)) => {
        // Logic to handle AMM functions
        let x = 100.0; // Replace with actual value
        let y = 200.0; // Replace with actual value
        let k = x * y;
        println!("Handling AMM functions with x: {}, y: {}, k: {}", x, y, k);
    }

    println!("Checking prices..."); 
