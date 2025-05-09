//! Example for deserializing ABI using `json_abi`.

use alloy::json_abi::JsonAbi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the contract abi.
    let path = std::env::current_dir()?.join("examples/advanced/examples/abi/SimpleLending.json");
    let contents = std::fs::read(path)?;
    let abi: JsonAbi = serde_json::from_slice(&contents)?;

    // Print deserialized ABI components
    println!("Deserialized ABI:");

    // Constructor
    if let Some(constructor) = &abi.constructor {
        println!("\n>> Constructor:");
        println!("  Inputs: {:?}", constructor.inputs);
        println!("  State mutability: {:?}", constructor.state_mutability);
    }

    println!("\n=========\n");

    // Functions
    println!("Functions:");
    for (name, functions) in &abi.functions {
        println!("\n>> {name}:");
        for function in functions {
            println!("    Inputs: {:?}", function.inputs);
            println!("    Outputs: {:?}", function.outputs);
            println!("    State mutability: {:?}", function.state_mutability);
        }
    }

    println!("\n=========\n");

    // Events
    println!("Events:");
    for (name, events) in &abi.events {
        println!("\n>> {name}:");
        for event in events {
            println!("    Inputs: {:?}", event.inputs);
            println!("    Anonymous: {}", event.anonymous);
        }
    }

    println!("\n=========\n");

    // Errors
    println!("Errors:");
    for (name, errors) in &abi.errors {
        println!(">> {name}:");
        for error in errors {
            println!("    Inputs: {:?}", error.inputs);
        }
    }

    println!("\n=========\n");

    // Example of working with a specific function
    if let Some(add_collateral) = abi.functions.get("addCollateral").and_then(|f| f.first()) {
        println!("Example: addCollateral() function exists!");
        println!("Inputs:");
        for input in &add_collateral.inputs {
            println!("  Name: {}, Type: {}", input.name, input.ty);
        }
    }

    Ok(())
}
