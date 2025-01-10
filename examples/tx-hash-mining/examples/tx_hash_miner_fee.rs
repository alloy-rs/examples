//! Example of mining a vanity Transaction hash using the max_fee_per_gas field.

use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    consensus::TxEnvelope,
    hex,
    signers::local::{
        coins_bip39::English,
        MnemonicBuilder, LocalSignerError,
    },

};
use eyre::Result;
use tokio;

async fn create_wallet() -> Result<PrivateKeySigner, LocalSignerError> {
    let wallet = MnemonicBuilder::<English>::default()
        .word_count(12)
        .derivation_path("m/44'/60'/0'/2/1")?
        .build_random()?;
    Ok(wallet)
}

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://eth.merkle.io".parse()?;

    let new_wallet = create_wallet()
        .await
        .unwrap();

    let wallet = EthereumWallet::from(new_wallet.clone());

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .on_http(rpc_url);

    let nonce = provider.get_transaction_count(new_wallet.address()).await?;
    let eip1559_est = provider.estimate_eip1559_fees(None).await?;

    let mut max_fee_per_gas = eip1559_est.max_fee_per_gas;
    println!("Starting BaseFee/max_fee_per_gas: {}", max_fee_per_gas);

    let mut tx_envelope;
    loop {
        let tx = TransactionRequest::default()
            .with_to("0xdEAD000000000000000042069420694206942069".parse()?)
            .with_nonce(nonce)
            .with_chain_id(1)
            .with_value(U256::from(0))
            .with_gas_limit(21_000)
            .with_max_priority_fee_per_gas(eip1559_est.max_priority_fee_per_gas)
            .with_max_fee_per_gas(max_fee_per_gas.into());

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
