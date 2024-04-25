//! Example of sending a EIP-1559 transaction with access list.
use alloy::{
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::{BlockId, TransactionRequest},
    sol,
};
use eyre::Result;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SimpleStorage,
    "examples/artifacts/SimpleStorage.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up anvil node.

    let anvil = Anvil::new().try_spawn()?;

    // Create a provider.
    let provider =
        ProviderBuilder::new().with_recommended_fillers().on_builtin(&anvil.endpoint()).await?;

    let alice = anvil.addresses()[0];
    let bob = anvil.addresses()[1];

    let contract_addr = SimpleStorage::deploy_builder(provider.clone(), "foo".to_string())
        .from(alice)
        .deploy()
        .await?;

    let contract = SimpleStorage::new(contract_addr, provider.clone());

    let set_value_call = contract.setValues("hello".to_string(), "world".to_string());
    let calldata = set_value_call.calldata().to_owned();

    let eip1559_fees = provider.estimate_eip1559_fees(None).await?;
    let tx = TransactionRequest::default()
        .from(bob)
        .to(contract_addr)
        .input(calldata.into())
        .max_fee_per_gas(eip1559_fees.max_fee_per_gas)
        .max_priority_fee_per_gas(eip1559_fees.max_priority_fee_per_gas);

    let access_list_with_gas_used = provider.create_access_list(&tx, BlockId::latest()).await?;

    let tx_with_access_list = tx.clone().access_list(access_list_with_gas_used.access_list);

    let pending_tx = provider.send_transaction(tx_with_access_list).await?;

    let _receipt = pending_tx.get_receipt().await?;

    let value = contract.getValue().call().await?;
    assert_eq!(value._0, "hello".to_string());

    Ok(())
}
