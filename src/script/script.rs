use std::rgb_core;

use create::Script;
use create::SwapSchema;
use create::PoolSchema;


use rgb_core::{
    contract::{
        Contract, Node, Output, Transition, TransitionType,
        Variables,
    },
    proof::Proof,
    schema::{
        AssetId, ContractId, OutputId,
        FieldType, GenesisSchema, Schema,
        TransitionSchema,
    },
    validation::{TxBuilder, TxValidator},
    Value,
};
use std::convert::TryFrom;

// Define the schema for the BTC/RGB asset swap contract
#[derive(Clone, Debug, PartialEq, Eq)]
struct SwapSchema {
    btc_input: FieldType,
    rgb_asset_input: FieldType,
    btc_output: FieldType,
    rgb_asset_output: FieldType,
    rate: FieldType,
}

impl SwapSchema {
    fn new() -> Self {
        Self {
            btc_input: "btc_input".into(),
            rgb_asset_input: "rgb_asset_input".into(),
            btc_output: "btc_output".into(),
            rgb_asset_output: "rgb_asset_output".into(),
            rate: "rate".into(),
        }
    }
}

// Define the swap contract
#[derive(Clone, Debug, PartialEq, Eq)]
struct SwapContract {}

impl Contract<SwapSchema> for SwapContract {
    fn transition(
        &self,
        _input_ids: Vec<OutputId>,
        inputs: Vec<Node>,
        _variables: &Variables,
        transition: &Transition,
        _schema: &SwapSchema,
    ) -> Result<Vec<Output>, String> {
        match transition.transition_type {
            TransitionType::Custom(ref action) => match action.as_str() {
                "swap" => {
                    // Get the BTC and RGB assets inputs
                    let btc_input = inputs
                        .iter()
                        .find(|n| n.field_type == "btc_input")
                        .ok_or("BTC input not found")?;
                    let usdt_input = inputs
                        .iter()
                        .find(|n| n.field_type == "rgb_asset_input")
                        .ok_or("RGB Asset input not found")?;

                    // Check that the inputs are valid
                    if btc_input.amount == Value::from(0)
                        || rgb_asset_input.amount == Value::from(0)
                        || btc_input.issuer != usdt_input.issuer
                        || btc_input.owner == usdt_input.owner
                    {
                        return Err("Invalid inputs".into());
                    }

                    // Calculate the exchange rate
                    let rate = rgb_asset_input.amount / btc_input.amount;

                    // Create BTC and RGB assets outputs with swapped amounts
                    let btc_output = Node {
                        field_type: "btc_output".into(),
                        value: Value::from(rgb_asset_input.amount),
                        issuer: btc_input.issuer.clone(),
                        owner: btc_input.owner.clone(),
                    };
                    let usdt_output = Node {
                        field_type: "rgb_asset_output".into(),
                        value: Value::from(btc_input.amount),
                        issuer: rgb_asset_input.issuer.clone(),
                        owner: rgb_asset_input.owner.clone(),
                    };

                    // Return the outputs and the exchange rate
                    Ok(vec![
                        Output::new(btc_output.clone()),
                        Output::new(rgb_asset_output.clone()),
                        Output::new(Node {
                            field_type: "rate".into(),
                            value: Value::from(rate),
                            issuer: btc_input.issuer.clone(),
                            owner: btc_input.owner.clone(),
                        }),
                    ])
                }
                _ => Err("Invalid action".into()),
            },
            _ => Err("Invalid transition type".into()),
        }
    }
}

// Define the schema for the liquidity pool contract
#[derive(Clone, Debug, PartialEq, Eq)]
struct PoolSchema {
    btc_pool_addr: FieldType,
    rgb_asset_pool_addr: FieldType,


// Define AluVM to scripting

#[derive(Clone, Debug,RgbIsa, AluLib), OsAssets,]
let code = [RgbIsa::Contract(ContractOp::PcVs(OS_ASSETS))];
let alu_lib = Lib::assemble(&code).unwrap();
let alu_id = alu_lib.id();
