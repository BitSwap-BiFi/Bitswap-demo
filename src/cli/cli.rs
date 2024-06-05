use lightning::Wallet;
use rgb_core::{contract::Contract, fungible::{Mint, OutpointValue, ValueSum}};
use rgb_core::{issue::Amount};
use rgb_contract::Cli;
use lightning::Channel;
use crate::Cli;
use rgbstd::interface::{Amount, ContractID, Mint, OutpointValue, ValueSum};
use std::{error::Error};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
     let rgb = RGB::new();
     let lightning = lightning:new():

    // Create a new RGB contract instance
    let contract = Contract::new();

    // Create a new wallet instance
    let mut wallet = Wallet::new();

     
    /// Network to use
    #[arg(short, long, default_value = "testnet")]
    network: String;

    loop {
        // Display the command prompt
        print!("> ");
        io::stdout().flush()?;

        // Read the user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // Parse the user input
        let args: Vec<&str> = input.trim().split(' ').collect();

        // Check the command and execute the corresponding action
        match args[0] {
            "swaps" => {
           let swap = swap::BTC();
           let swap = swap::RGB20();
            }
            "enable_channel" => {
                 let channel = open::new();
               
            }
            "disable_channel" => {
               let channel = close::new();

               
            }
            "enter_liquidity" => {
               let mut input = String::new();
               let input = asset::BTC:();
               let input = asset::RGB20();
               
            }
            "exit_liquidity" => {
               let mut output = String::new();
               let output = asset::BTC:();
               let output = asset::RGB20();
            }
            "exit" => {
                // Exit the program
                break;
            }
            _ => {
                // Invalid command
                println!("Invalid command");
            }
        }
    }

    Ok(())
}
