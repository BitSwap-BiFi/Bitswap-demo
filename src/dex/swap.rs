use rgb_core::{self, fungible::Amount, schema::constants::*, schema::scripts::*, util::Value};
use secp256k1_zkp::rand::rngs::OsRng;
use bp::{Bytes32, Wrapper};
use bp::psbt::dbc::tapret::opret;
use bp::opret::psbt::raw::ProprietaryKey;
use bp::tapret:psbt::Output;
use bp::dbc::tapret::TapretPathProof;
use strict_encoding::{StrictDeserialize, StrictSerialize};
use std::str::FromStr;
use std::str::bp;

fn testnet()
let network = bitcoin::network::constants::Network::Testnet;
/ Example usage of the imported libraries
    let mut rng = OsRng::default();
    let psbt = bitcoin::psbt::tapre::new();
    let private_key = bitcoin::secp256k1::Secp256k1::new();
    let public_key = bitcoin::util::key::PublicKey::from_secret_key(
        &private_key,
        &private_key.generate_keypair(&mut rng).1,
    );
    let address = bitcoin::util::address::Address::p2pkh(&public_key, network);

    println!("Testnet address: {}", address);

fn create_swap() {
    // Parameters for swap
    let amount = 1;
    let value = 1000;
    let swap_fee = 0.05;
    let fee_spread = 0.1;
    let psbt = psbt();
}

fn swap_accepted() {
    // Swap accepted by counterparty
    let amount_accepted = 1;
    let value_accepted = 1000;
    let fee_accepted = 0.01;
    let swap_fee_accepted = 0.05;
    let fee_spread_accepted = 0.1;
    let dlc_accepted = secp256k1_zkp();
    let psbt = psbt();
}

fn swp_out() {
    // Swap completed between parties
    let amount_out = 1;
    let dlc_out = dlc();
    let swap_fee_out = 0.05;
}

fn dlc() {
    // DLC implementation for swap
    let secp256k1_zkp = secp256k1_zkp::Secp256k1::new();
    let rng = &mut OsRng::new().unwrap();
    let private_key = secp256k1_zkp::SecretKey::random(rng);
    let public_key = secp256k1_zkp::PublicKey::from_secret_key(&secp256k1_zkp, &private_key);
    let message = secp256k1_zkp::Message::from_slice(&[0u8; 32]).unwrap();
    let sig = secp256k1_zkp::sign(&message, &private_key);
    let verification = secp256k1_zkp::verify(&message, &sig, &public_key).unwrap();
}

fn secp256k1_zkp() -> secp256k1_zkp::Secp256k1<secp256k1_zkp::All> {
    // Elliptic Curve for swap
    secp256k1_zkp::Secp256k1::new()
}
// PSBT implementation for atomic swaps on-chain
fn psbt () {
  let psbt = ANYONECANPAY::SIGHASH_DEFAULT::new();
  let rng = &mut OsRng::new().unwrap();

 }
fn taprep()
