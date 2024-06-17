//! This example demonstrates how to interact with a contract that is already deployed onchain using
//! the `ContractInstance` interface.
use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    network::{Ethereum, TransactionBuilder},
    primitives::{hex, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    transports::http::{Client, Http},
};
use eyre::Result;

/**
* solc v0.8.26; solc a.sol --via-ir --optimize --bin
* contract Counter {
       uint256 public number;

       function setNumber(uint256 newNumber) public {
           number = newNumber;
       }

       function increment() public {
           number++;
       }
   }
*
*/
#[tokio::main]
async fn main() -> Result<()> {
    let provider = ProviderBuilder::new().with_recommended_fillers().on_anvil_with_wallet();

    // Deploy the Counter contract
    let bytecode = hex::decode("6080806040523460135760df908160198239f35b600080fdfe6080806040526004361015601257600080fd5b60003560e01c9081633fb5c1cb1460925781638381f58a146079575063d09de08a14603c57600080fd5b3460745760003660031901126074576000546000198114605e57600101600055005b634e487b7160e01b600052601160045260246000fd5b600080fd5b3460745760003660031901126074576020906000548152f35b34607457602036600319011260745760043560005500fea2646970667358221220e978270883b7baed10810c4079c941512e93a7ba1cd1108c781d4bc738d9090564736f6c634300081a0033")?;
    let deploy_tx = TransactionRequest::default().with_deploy_code(bytecode);

    let contract_address =
        provider.send_transaction(deploy_tx).await?.get_receipt().await?.contract_address.unwrap();

    // Get the contract abi
    let path = std::env::current_dir()?.join("examples/contracts/examples/artifacts/Counter.json");

    // Read the artifact which contains `abi`, `bytecode`, `deployedBytecode`,and `metadata`
    let artifact = std::fs::read(path).unwrap();
    let json: serde_json::Value = serde_json::from_slice(&artifact)?;

    // Get `abi` from the artifact
    let abi_val = json.get("abi").unwrap();
    let abi = serde_json::from_str(&abi_val.to_string())?;

    // Create a new `ContractInstance` of the Counter contract from the abi
    let counter_instance: ContractInstance<Http<Client>, _, Ethereum> =
        ContractInstance::new(contract_address, provider.clone(), Interface::new(abi));

    // Interact with the contract
    assert_eq!(counter_instance.abi().functions().count(), 3);

    // Read
    let init_val = counter_instance.function("number", &[])?.call().await?;

    // Get the Uint value from the result
    let init_val = init_val.first().unwrap().as_uint().unwrap().0;

    assert_eq!(U256::from(0), init_val);

    // Increment
    let incr_receipt =
        counter_instance.function("increment", &[])?.send().await?.get_receipt().await?;

    assert!(incr_receipt.status());

    let incremented_val = counter_instance.function("number", &[])?.call().await?;

    let incremented_val = incremented_val.first().unwrap().as_uint().unwrap().0;

    assert_eq!(U256::from(1), incremented_val);

    // Set the number
    let set_val = DynSolValue::from(U256::from(100));

    let set_receipt =
        counter_instance.function("setNumber", &[set_val])?.send().await?.get_receipt().await?;

    assert!(set_receipt.status());

    let set_val = counter_instance.function("number", &[])?.call().await?;

    let set_val = set_val.first().unwrap().as_uint().unwrap().0;

    assert_eq!(U256::from(100), set_val);

    // Try calling a function that does not exist
    let decr_call = counter_instance.function("decrement", &[]).unwrap_err();

    assert!(decr_call.to_string().contains("function decrement does not exist"));

    Ok(())
}
