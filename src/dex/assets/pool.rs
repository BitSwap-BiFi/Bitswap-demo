use create::{Pool, Contract, UserChoice};
use rgb_core::{Contract, ExecutionResult};
use aluvm::{Executor, Value};
use rgbstd::{Amount, ContractData, DivisibleAssetSpec, StandardTypes, Timestamp};

fn main() {
    // Set up your RGB contract and polling logic
    let contract = Contract::new();

    // User's choice: BTC or RGB asset
    let user_choice = "BTC";
    let user_choice = "RGB_ASSET";

    // Execute the contract using AluVM
    let mut executor = Executor::new(&contract);
    let mut input = vec![Value::String(user_choice.to_owned())];
    let result = executor.execute("poll", &mut input);

    // Handle the execution result
    match result {
        Ok(ExecutionResult::Value(result)) => {
            // User has earned either RGB Asset or BTC
            println!("Earned: {}", result);
        }
        Ok(ExecutionResult::None) => {
            // Invalid or unrecognized user choice
            println!("Invalid choice");
        }
        Err(err) => {
            // Handle any execution errors
            eprintln!("Execution error: {:?}", err);
        }
    }
}
