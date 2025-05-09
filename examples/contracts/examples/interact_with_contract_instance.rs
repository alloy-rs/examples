//! This example demonstrates how to interact with a contract that is already deployed onchain using
//! the `ContractInstance` interface.

use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    network::TransactionBuilder,
    primitives::{hex, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let provider = ProviderBuilder::new().connect_anvil_with_wallet();

    // Deploy the `Counter` contract from bytecode at runtime.
    let bytecode = hex::decode(
        // solc v0.8.26; solc Counter.sol --via-ir --optimize --bin
        //
        // contract Counter {
        //     uint256 public number;
        //
        //     function setNumber(uint256 newNumber) public {
        //         number = newNumber;
        //     }
        //
        //     function increment() public {
        //         number++;
        //     }
        // }
        "6080806040523460135760df908160198239f35b600080fdfe6080806040526004361015601257600080fd5b60003560e01c9081633fb5c1cb1460925781638381f58a146079575063d09de08a14603c57600080fd5b3460745760003660031901126074576000546000198114605e57600101600055005b634e487b7160e01b600052601160045260246000fd5b600080fd5b3460745760003660031901126074576020906000548152f35b34607457602036600319011260745760043560005500fea2646970667358221220e978270883b7baed10810c4079c941512e93a7ba1cd1108c781d4bc738d9090564736f6c634300081a0033"
    )?;
    let tx = TransactionRequest::default().with_deploy_code(bytecode);

    let contract_address = provider
        .send_transaction(tx)
        .await?
        .get_receipt()
        .await?
        .contract_address
        .expect("Failed to get contract address");

    // Get the contract ABI.
    let path = std::env::current_dir()?.join("examples/contracts/examples/artifacts/Counter.json");

    // Read the artifact which contains `abi`, `bytecode`, `deployedBytecode` and `metadata`.
    let artifact = std::fs::read(path).expect("Failed to read artifact");
    let json: serde_json::Value = serde_json::from_slice(&artifact)?;

    // Get `abi` from the artifact.
    let abi_value = json.get("abi").expect("Failed to get ABI from artifact");
    let abi = serde_json::from_str(&abi_value.to_string())?;

    // Create a new `ContractInstance` of the `Counter` contract from the abi
    let contract = ContractInstance::new(contract_address, provider.clone(), Interface::new(abi));

    // Set the number to 42.
    let number_value = DynSolValue::from(U256::from(42));
    let tx_hash = contract.function("setNumber", &[number_value])?.send().await?.watch().await?;

    println!("Set number to 42: {tx_hash}");

    // Increment the number to 43.
    let tx_hash = contract.function("increment", &[])?.send().await?.watch().await?;

    println!("Incremented number: {tx_hash}");

    // Retrieve the number, which should be 43.
    let number_value = contract.function("number", &[])?.call().await?;
    let number = number_value.first().unwrap().as_uint().unwrap().0;
    assert_eq!(U256::from(43), number);

    println!("Retrieved number: {number}");

    // Try calling a function that does not exist.
    let unknown_function = contract.function("decrement", &[]).unwrap_err();
    assert!(unknown_function.to_string().contains("function decrement does not exist"));

    Ok(())
}
