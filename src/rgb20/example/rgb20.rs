use amplify::hex::FromHex;
use bp::dbc::Method;
use bp::{Outpoint, Txid};
use ifaces::Rgb20;
use rgbstd::containers::{FileContent, Kit};
use rgbstd::interface::{FilterIncludeAll, FungibleAllocation};
use rgbstd::invoice::Precision;
use rgbstd::persistence::{MemIndex, MemStash, MemState, Stock};
use schemata::dumb::DumbResolver;
use schemata::NonInflatableAsset;

#[rustfmt::skip]
fn main() {
    let beneficiary_txid = 
        Txid::from_hex("14295d5bb1a191cdb6286dc0944df938421e3dfcbf0811353ccac4100c2068c5").unwrap();
    let beneficiary = Outpoint::new(beneficiary_txid, 1);

    let contract = Rgb20::testnet::<NonInflatableAsset>("ssi:anonymous","TEST", "Test asset", None, Precision::CentiMicro)
        .expect("invalid contract data")
        .allocate(Method::TapretFirst, beneficiary, 1_000_000_000_00u64)
        .expect("invalid allocations")
        .issue_contract()
        .expect("invalid contract data");

    let contract_id = contract.contract_id();

    eprintln!("{contract}");
    contract.save_file("test/rgb20-example.rgb").expect("unable to save contract");
    contract.save_armored("test/rgb20-example.rgba").expect("unable to save armored contract");

    let kit = Kit::load_file("schemata/NonInflatableAssets.rgb").unwrap().validate().unwrap();

    // Let's create some stock - an in-memory stash and inventory around it:
    let mut stock = Stock::<MemStash, MemState, MemIndex>::default();
    stock.import_kit(kit).expect("invalid issuer kit");
    stock.import_contract(contract, &mut DumbResolver).unwrap();

    // Reading contract state through the interface from the stock:
    let contract = stock.contract_iface_class::<Rgb20>(contract_id).unwrap();
    let contract = Rgb20::from(contract);
    let allocations = contract.fungible("assetOwner", &FilterIncludeAll).unwrap();
    eprintln!("\nThe issued contract data:");
    eprintln!("{}", serde_json::to_string(&contract.spec()).unwrap());

    for FungibleAllocation  { seal, state, witness, .. } in allocations {
        eprintln!("amount={state}, owner={seal}, witness={witness}");
    }
    eprintln!("totalSupply={}", contract.total_supply());
}