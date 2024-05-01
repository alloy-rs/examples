//! Example showing how to send a 4844 tx.

use alloy::{
    consensus::{SidecarBuilder, SimpleCoder},
    network::TransactionBuilder,
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::TransactionRequest,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().args(["--hardfork", "cancun"]).spawn();
    let provider = ProviderBuilder::new().on_builtin(&anvil.endpoint()).await?;

    let from = anvil.addresses()[0];
    let to = anvil.addresses()[1];

    let sidecar: SidecarBuilder<SimpleCoder> =
        SidecarBuilder::from_slice("Blobs are fun!".as_bytes());

    let sidecar = sidecar.build()?;

    let gas_price = provider.get_gas_price().await?;
    let eip1559_est = provider.estimate_eip1559_fees(None).await?;
    let tx = TransactionRequest::default()
        .with_from(from)
        .with_to(to)
        .with_nonce(0)
        .with_max_fee_per_blob_gas(gas_price)
        .with_max_fee_per_gas(eip1559_est.max_fee_per_gas)
        .with_max_priority_fee_per_gas(eip1559_est.max_priority_fee_per_gas)
        .with_blob_sidecar(sidecar);

    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    assert!(receipt.blob_gas_used.unwrap() > 0);

    Ok(())
}
