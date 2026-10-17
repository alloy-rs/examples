//! Example of sharing one signer across providers for multiple EVM-compatible networks.
//!
//! Set `RPC_URLS` to comma-separated Ethereum, Optimism, Arbitrum, or other EVM endpoints,
//! and `PRIVATE_KEY` to an account funded on each network. Use testnets for this example.
//! Each provider sends a zero-value transaction back to the signer, spending gas if included.
//! Chain IDs, nonces, and fees are filled independently for each provider.

use alloy::{
    network::{AnyNetwork, EthereumWallet, TransactionBuilder},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use example_support::{required_env, rpc_urls};
use eyre::{ensure, Result};
use futures_util::future::try_join_all;

#[tokio::main]
async fn main() -> Result<()> {
    let signer = required_env("PRIVATE_KEY")?.parse::<PrivateKeySigner>()?;
    let sender = signer.address();
    let wallet = EthereumWallet::from(signer);

    let mut providers = Vec::new();
    for url in rpc_urls()? {
        // Cloning the wallet shares its signer. Each provider has its own fillers, so the
        // nonce and chain ID from one network cannot be reused on another network.
        providers.push(
            ProviderBuilder::new()
                .network::<AnyNetwork>()
                .wallet(wallet.clone())
                .connect_http(url.parse()?),
        );
    }

    // Check all endpoints before sending, and avoid racing two nonce fillers on the same chain.
    let chain_ids = try_join_all(providers.iter().map(|provider| provider.get_chain_id())).await?;
    for (index, chain_id) in chain_ids.iter().enumerate() {
        ensure!(!chain_ids[..index].contains(chain_id), "RPC_URLS must use distinct chains");
    }

    // Join transactions from independent networks while reusing the same signer.
    try_join_all(providers.iter().zip(chain_ids).map(|(provider, chain_id)| async move {
        let tx = TransactionRequest::default().with_from(sender).with_to(sender);
        let receipt = provider.send_transaction(tx.into()).await?.get_receipt().await?;
        ensure!(receipt.status(), "Transaction reverted on chain {chain_id}");
        assert_eq!(receipt.from, sender);
        assert_eq!(receipt.to, Some(sender));
        println!("Chain {chain_id}: {}", receipt.transaction_hash);

        // AnyNetwork preserves extra receipt fields, such as Optimism's l1Fee and
        // Arbitrum's gasUsedForL1. They need not be present on every EVM network.
        println!("Network-specific receipt fields: {:?}", receipt.other);
        Ok::<_, eyre::Report>(())
    }))
    .await?;

    Ok(())
}
