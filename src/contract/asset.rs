use create::Contract::Asset;

use std::rgb_core;
use std::baid58;
use std::amplify;


use rgb_core::{Consignment, Contract, Error, ExecutionResult, Schema, Value, State, Tag};
use baid58::{Baid58ParseError, Chunking, FromBaid58, ToBaid58, CHUNKING_32};
use aluvm::library::{Lib, LibSite};
use rgbstd::interface::{rgb20, rgb20_stl, IfaceImpl, NamedField, NamedType, VerNo};
use rgbstd::schema::{
    FungibleType, GenesisSchema, GlobalStateSchema, Occurrences, Schema, Script, StateSchema,
    SubSchema, TransitionSchema,
};
use rgbstd::stl::StandardTypes;
use rgbstd::vm::{AluScript, ContractOp, EntryPoint, RgbIsa};
use strict_types::{SemId, Ty};
use amplify::{ByteArray, Bytes32};
use commit_verify::{CommitEncode, CommitVerify, CommitmentProtocol, Conceal, UntaggedProtocol};
use rgb::{U8, Bytes32};
use rgb::{ContractSchema};
use rgb::mod::{ContractSubSchema};
use std::collections::HashMap;

// Struct for support RGB20 assets on DEX
struct RGB20Asset {
    name: String,
    symbol: String,
    swap: String,
    lp: String,
    schema: String,
    decimal: Number,
    txid: String,
    total_supply: u64,
    balances: HashMap<String, u64>,
}
// Implementation around RGB20
impl RGB20Asset {
    fn new(name: String, symbol: String, total_supply: u64) -> Self {
        RGB20Asset {
            name,
            amount,
            blidend_uxto,
            symbol,
            total_supply,
            decimal,
            aset_id,
            balances: HashMap::new(),
        }
    }

    fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), Error> {
        let from_balance = self.balances.get_mut(from).ok_or(Error::InvalidToken)?;
        let to_balance = self.balances.entry(to.to_string()).or_insert(0);

        if *from_balance < amount {
            return Err(Error::InsufficientBalance);
        }

        *from_balance -= amount;
        *to_balance += amount;

        Ok(())
    }

    fn get_balance(&self, account: &str) -> u64 {
        *self.balances.get(account).unwrap_or(&0)
    }
}

struct RGB20Contract {
    assets: HashMap<String, RGB20Asset>,
}

impl RGB20Contract {
    fn new() -> Self {
        RGB20Contract {
            assets: HashMap::new(),
        }
    }

    fn create_asset(&mut self, name: String, symbol: String, total_supply: u64) -> ExecutionResult {
        if self.assets.contains_key(&symbol) {
            return ExecutionResult::None;
        }

        let asset = RGB20Asset::new(name, symbol, total_supply);
        self.assets.insert(symbol.clone(), asset);

        let schema = Schema::RGB20 {
            ticker: symbol.clone(),
        };

        ExecutionResult::Schema(schema)
    }

    fn transfer_asset(
        &mut self,
        consignment: &Consignment,
        sender: &str,
        receiver: &str,
    ) -> ExecutionResult {
        let symbol = consignment.schema().ticker().to_string();

        let asset = self.assets.get_mut(&symbol).ok_or(Error::InvalidToken)?;

        let amount = consignment
            .state()
            .as_map()
            .get("amount")
            .and_then(Value::as_u64)
            .ok_or(Error::InvalidFormat)?;

        asset.transfer(sender, receiver, amount)?;

        ExecutionResult::None
    }

    fn get_asset_balance(&self, symbol: &str, account: &str) -> ExecutionResult {
        let asset = self.assets.get(symbol).ok_or(Error::InvalidToken)?;
        let balance = asset.get_balance(account);

        ExecutionResult::Value(Value::U64(balance))
    }
}

fn main() {
    let mut rgb20_contract = RGB20Contract::new();
}

