use rgb_core::{Contract, ExecutionResult};
use aluvm::{Executor, Value};

fn main() {
    // Set up your RGB contract and polling logic
    let contract = Contract::new();

    // Simulate a user's choice: BTC or USDT
    let user_choice = "BTC";

    // Execute the contract using AluVM
    let mut executor = Executor::new(&contract);
    let mut input = vec![Value::String(user_choice.to_owned())];
    let result = executor.execute("poll", &mut input);

    // Handle the execution result
    match result {
        Ok(ExecutionResult::Value(result)) => {
            // User has earned either USDT or BTC
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
