#[post("/swap")]
fn swap(from_token: String, to_token: String, amount: f64) -> String {
    let sdk = RGB::new("regtest", "https://explorer.your-domain.com").unwrap();
    let contract_id = "your-contract-id";
    let contract = sdk.load_contract(contract_id).unwrap();
    
    let result = contract.call("swap", &json!({
        "fromToken": from_token,
        "toToken": to_token,
        "amount": amount
    }));
    
    match result {
        Ok(tx) => tx.to_string(),
        Err(e) => e.to_string()
    }
}
