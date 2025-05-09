//! Example of sending a EIP-1559 transaction with access list.

use alloy::{
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    sol,
};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SimpleStorage,
    "examples/artifacts/SimpleStorage.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let provider = ProviderBuilder::new().connect_anvil();

    // Create two users, Alice and Bob.
    let accounts = provider.get_accounts().await?;
    let alice = accounts[0];
    let bob = accounts[1];

    // Deploy the `SimpleStorage` contract.
    let contract_address = SimpleStorage::deploy_builder(provider.clone(), "initial".to_string())
        .from(alice)
        .deploy()
        .await?;
    let contract = SimpleStorage::new(contract_address, provider.clone());

    // Build a transaction to set the values of the contract.
    // The `from` field is automatically filled to the first signer's address (Alice).
    let set_value_call = contract.setValues("hello".to_string(), "world".to_string());
    let calldata = set_value_call.calldata().to_owned();
    let tx = TransactionRequest::default().from(bob).to(contract_address).input(calldata.into());

    // Create an access list for the transaction.
    let access_list_with_gas_used = provider.create_access_list(&tx).await?;

    // Add the access list to the transaction.
    let tx_with_access_list = tx.access_list(access_list_with_gas_used.access_list);

    // Send the transaction with the access list.
    let tx_hash = provider.send_transaction(tx_with_access_list).await?.watch().await?;

    println!("Transaction hash: {tx_hash}");

    // Check the value of the contract.
    let value = contract.getValue().call().await?;

    assert_eq!(value, "hello");

    Ok(())
}
