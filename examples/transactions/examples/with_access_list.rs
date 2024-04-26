//! Example of sending a EIP-1559 transaction with access list.

use alloy::{
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::{BlockId, TransactionRequest},
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
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().try_spawn()?;

    // Create a provider.
    let provider =
        ProviderBuilder::new().with_recommended_fillers().on_builtin(&anvil.endpoint()).await?;

    // Create two users, Alice and Bob.
    let alice = anvil.addresses()[0];
    let bob = anvil.addresses()[1];

    // Deploy the `SimpleStorage` contract.
    let contract_address = SimpleStorage::deploy_builder(provider.clone(), "initial".to_string())
        .from(alice)
        .deploy()
        .await?;
    let contract = SimpleStorage::new(contract_address, provider.clone());

    // Build a transaction to set the values.
    let set_value_call = contract.setValues("hello".to_string(), "world".to_string());
    let calldata = set_value_call.calldata().to_owned();

    let eip1559_fees = provider.estimate_eip1559_fees(None).await?;
    let tx = TransactionRequest::default()
        .from(bob)
        .to(contract_address)
        .input(calldata.into())
        .max_fee_per_gas(eip1559_fees.max_fee_per_gas)
        .max_priority_fee_per_gas(eip1559_fees.max_priority_fee_per_gas);

    // Create an access list for the transaction.
    let access_list_with_gas_used = provider.create_access_list(&tx, BlockId::latest()).await?;

    // Add the access list to the transaction.
    let tx_with_access_list = tx.access_list(access_list_with_gas_used.access_list);

    // Send the transaction with the access list.
    provider.send_transaction(tx_with_access_list).await?.get_receipt().await?;

    // Check the value of the contract.
    let value = contract.getValue().call().await?;

    assert_eq!(value._0, "hello".to_string());

    Ok(())
}
