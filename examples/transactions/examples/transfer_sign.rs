use alloy::network::{eip2718::Encodable2718, TransactionBuilder};
use alloy::primitives::utils::parse_units;
use alloy::providers::Provider;
use alloy::{
    network::EthereumSigner, node_bindings::Anvil, primitives::U256, providers::ProviderBuilder,
    rpc::types::eth::TransactionRequest, signers::wallet::LocalWallet,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().try_spawn()?;

    // Set up signer from the first default Anvil account (Alice).
    let signer: LocalWallet = anvil.keys()[0].clone().into();
    let destination: LocalWallet = anvil.keys()[1].clone().into();

    let rpc_url = anvil.endpoint().parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url)?;

    let tx = TransactionRequest {
        from: Some(signer.address()),
        to: Some(destination.address()),
        nonce: Some(provider.get_transaction_count(signer.address(), 0.into()).await?),
        chain_id: Some(provider.get_chain_id().await?),
        value: Some(U256::from(42)),
        gas: Some(21000),
        max_priority_fee_per_gas: Some(parse_units("0.1", "gwei").unwrap().get_absolute().to()),
        max_fee_per_gas: Some(parse_units("30", "gwei").unwrap().get_absolute().to()),
        ..Default::default()
    };

    let to_send = tx.build(&EthereumSigner::from(signer)).await?;

    let out = to_send.encoded_2718();

    let receipt = provider.send_raw_transaction(&out).await?.get_receipt().await?;

    println!("Send transaction receipt: {receipt:?}");

    Ok(())
}
