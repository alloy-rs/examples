//! Example of mining a vanity Transaction hash using the `max_fee_per_gas` field.
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

    let mut max_fee_per_gas = eip1559_est.max_fee_per_gas;
    println!("Starting BaseFee/max_fee_per_gas: {}", max_fee_per_gas);

    let mut tx_envelope;
    loop {
        let tx = TransactionRequest::default()
            .with_to("0xdEAD000000000000000042069420694206942069".parse()?)
            .with_nonce(1)
            .with_chain_id(1)
            .with_value(U256::from(0))
            .with_gas_limit(21_000)
            .with_max_priority_fee_per_gas(eip1559_est.max_priority_fee_per_gas)
            .with_max_fee_per_gas(max_fee_per_gas);

        tx_envelope = tx.build(&wallet).await?;

        if let TxEnvelope::Eip1559(ref signed_tx) = tx_envelope {
            let tx_hash = hex::encode(signed_tx.hash());

            if tx_hash.starts_with("dead") {
                println!("Found a transaction hash starting with prefix: {:?}", tx_hash);
                println!("max fee per gas: {}", max_fee_per_gas);
                break;
            }
        }
        max_fee_per_gas += 1;
    }
    Ok(())
}
