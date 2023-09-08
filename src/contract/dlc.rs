use std::rgb_core
use std::rust_dlc

use rgb_core::Node;
use rust_dlc::{ContractOracle, ContractDescriptor};

fn main() {
    // Establish connection with RGB Node
    let node = Node::new("http://rgb-node-url:port");

    // Create a new DLC contract
    let mut dlc = ContractDescriptor::new();
    dlc.set_oracle(&node);

fn oracle

fn swap

fn lp

fn price
