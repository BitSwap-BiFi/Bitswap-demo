let btc_contract_id = ContractId::from_hex("YOUR_BTC_CONTRACT_ID").expect("Invalid contract ID");
let usdt_contract_id = ContractId::from_hex("YOUR_USDT_CONTRACT_ID").expect("Invalid contract ID");

node.register::<Asset>(btc_contract_id, Asset::BTC);
node.register::<Asset>(usdt_contract_id, Asset::USDT);

