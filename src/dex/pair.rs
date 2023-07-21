use rgb::traits::Validate;
use rgb::value::Value;
use rgb::{ContractId, Consignment, Genesis, Node, NodeId};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Asset {
    BTC,
    USDT,
    RGBTC,
    DCHF,
    RGBEX,      
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
}
fn main() {
    let node_id = NodeId::from_hex("YOUR_NODE_ID").expect("Invalid node ID");
    let mut node = Node::new(node_id);
}

