//! Example of mining a vanity Transaction hash using the `gas` field.

/// Extra gas is refunded to the sender. This can be used to mine a tx hash that starts with a
/// specific prefix by increasing the gas limit from X until the tx hash starts with the
/// desired prefix.
use alloy::{
    consensus::TxEnvelope,
    hex,
    network::TransactionBuilder,
    primitives::U256,
    providers::{Provider, ProviderBuilder, WalletProvider},
    rpc::types::TransactionRequest,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = ProviderBuilder::new().on_anvil_with_wallet();
    let wallet = provider.wallet();
    let eip1559_est = provider.estimate_eip1559_fees(None).await?;

    let mut tx_envelope;
    let mut gas: u64 = 100_000;

    loop {
        let tx = TransactionRequest::default()
            .with_to("0xdEAD000000000000000042069420694206942069".parse()?)
            .with_nonce(1)
            .with_chain_id(1)
            .with_value(U256::from(0))
            .with_gas_limit(gas)
            .with_max_priority_fee_per_gas(eip1559_est.max_priority_fee_per_gas)
            .with_max_fee_per_gas(eip1559_est.max_fee_per_gas);

        tx_envelope = tx.build(&wallet).await?;

        if let TxEnvelope::Eip1559(ref signed_tx) = tx_envelope {
            let tx_hash = hex::encode(signed_tx.hash());
            if tx_hash.starts_with("dead") {
                println!("Found a transaction hash starting with prefix: {:?}", tx_hash);
                println!("Gas used: {}", gas);
                break;
            }
        }
        gas += 1;
    }

    Ok(())
}
