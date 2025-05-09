//! Instantiate a basic provider without any fillers or layers.

use eyre::Result;

use alloy::{
    network::TransactionBuilder,
    node_bindings::Anvil,
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Spawn an Anvil instance
    // Make sure `anvil` is in $PATH
    let anvil = Anvil::new().try_spawn()?;
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let alice = signer.address();

    let provider = ProviderBuilder::new()
        // Disable the recommended fillers that are enabled by default
        .disable_recommended_fillers()
        // Add the signer to the provider for signing transactions
        .wallet(signer)
        .connect_http(anvil.endpoint().parse()?);

    let bob = Address::from([0x42; 20]);
    let fees = provider.estimate_eip1559_fees().await?;
    let nonce = provider.get_transaction_count(alice).await?;
    let chain_id = provider.get_chain_id().await?;

    let tx = TransactionRequest::default()
        .with_value(U256::from(1))
        .with_chain_id(chain_id)
        .with_from(alice)
        .with_nonce(nonce)
        .with_max_fee_per_gas(fees.max_fee_per_gas)
        .with_max_priority_fee_per_gas(fees.max_priority_fee_per_gas)
        .with_gas_limit(21000)
        .with_to(bob)
        .with_value(U256::from(1));

    let bob_balance_before = provider.get_balance(bob).await?;
    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;
    assert!(receipt.status(), "Transaction failed");
    let bob_balance_after = provider.get_balance(bob).await?;
    println!("Balance before: {bob_balance_before}\nBalance after: {bob_balance_after}");

    Ok(())
}
