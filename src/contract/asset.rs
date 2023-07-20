use rgb_core::{Consignment, Contract, Error, ExecutionResult, Schema, Value};
use std::collections::HashMap;

struct RGB20Asset {
    name: String,
    symbol: String,
    total_supply: u64,
    balances: HashMap<String, u64>,
}

impl RGB20Asset {
    fn new(name: String, symbol: String, total_supply: u64) -> Self {
        RGB20Asset {
            name,
            symbol,
            total_supply,
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

    // Create a new RGB20 asset
    let asset_name = "Tether".to_string();
    let asset_symbol = "USDT".to_string();
    let total_supply = 1000000;
    let asset_creation_result = rgb20_contract.create_asset(asset_name, asset_symbol, total_supply);
    println!("Asset creation result: {:?}", asset_creation_result);

    // Transfer the asset from one account to another
    let consignment = Consignment::from_state_json(r#"{"amount": 100}"#).unwrap();
    let sender = "account1";
    let receiver = "account2";
    let transfer_result = rgb20_contract.transfer_asset(&consignment, sender, receiver);
    println!("Asset transfer result: {:?}", transfer_result);

    // Get the balance of an account for a specific asset
    let balance_result = rgb20_contract.get_asset_balance("USDT", "account2");
    println!("Asset balance result: {:?}", balance_result);
}

fn main() {
    let mut rgb20_contract = RGB20Contract::new();

    // Create a new RGB20 asset
    let asset_name = "Wrapped Native BTC".to_string();
    let asset_symbol = "RGBTC".to_string();
    let total_supply = 0;
    let asset_creation_result = rgb20_contract.create_asset(asset_name, asset_symbol, total_supply);
    println!("Asset creation result: {:?}", asset_creation_result);

    // Transfer the asset from one account to another
    let consignment = Consignment::from_state_json(r#"{"amount": 0}"#).unwrap();
    let sender = "account1";
    let receiver = "account2";
    let transfer_result = rgb20_contract.transfer_asset(&consignment, sender, receiver);
    println!("Asset transfer result: {:?}", transfer_result);

    // Get the balance of an account for a specific asset
    let balance_result = rgb20_contract.get_asset_balance("RGBTC", "account2");
    println!("Asset balance result: {:?}", balance_result);
}
fn main() {
    let mut rgb20_contract = RGB20Contract::new();

    // Create a new RGB20 asset
    let asset_name = "Digital Swiss Franc".to_string();
    let asset_symbol = "dCHF".to_string();
    let total_supply = 0;
    let asset_creation_result = rgb20_contract.create_asset(asset_name, asset_symbol, total_supply);
    println!("Asset creation result: {:?}", asset_creation_result);

    // Transfer the asset from one account to another
    let consignment = Consignment::from_state_json(r#"{"amount": 0}"#).unwrap();
    let sender = "account1";
    let receiver = "account2";
    let transfer_result = rgb20_contract.transfer_asset(&consignment, sender, receiver);
    println!("Asset transfer result: {:?}", transfer_result);

    // Get the balance of an account for a specific asset
    let balance_result = rgb20_contract.get_asset_balance("dCHF", "account2");
    println!("Asset balance result: {:?}", balance_result);
}

fn main() {
    let mut rgb20_contract = RGB20Contract::new();

    // Create a new RGB20 asset
    let asset_name = " RGBexDAO token".to_string();
    let asset_symbol = "RGBEX".to_string();
    let total_supply = 0;
    let asset_creation_result = rgb20_contract.create_asset(asset_name, asset_symbol, total_supply);
    println!("Asset creation result: {:?}", asset_creation_result);

    // Transfer the asset from one account to another
    let consignment = Consignment::from_state_json(r#"{"amount": 0}"#).unwrap();
    let sender = "account1";
    let receiver = "account2";
    let transfer_result = rgb20_contract.transfer_asset(&consignment, sender, receiver);
    println!("Asset transfer result: {:?}", transfer_result);

    // Get the balance of an account for a specific asset
    let balance_result = rgb20_contract.get_asset_balance("RGBEX", "account2");
    println!("Asset balance result: {:?}", balance_result);
}
