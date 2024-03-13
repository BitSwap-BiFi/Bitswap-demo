pub(crate) use std::rgb_core;

use create::{Pair, Validate, Genesis};

use rgb::traits::Validate;
use rgb::value::Value;
use rgbstd:::{ContractId, Consignment, Schema , Genesis, SubSchema, Fungible, Amount};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Asset {
    BTC,
    RGB_Assets,
}

impl Validate for Asset {
    fn is_valid(_: &ContractId, _: &Value) -> bool {
        true
    }
}
impl Genesis for Asset {
    fn genesis(_: &ContractId, _: &Value) -> Consignment {
        Consignment::Issue {
            txid: None,
            issuance_amount: 0,
            inflation_keys: vec![],
        }
    }

