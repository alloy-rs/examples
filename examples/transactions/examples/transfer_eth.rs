use alloy::{
    network::Ethereum,
    node_bindings::Anvil,
    primitives::U256,
    providers::{Provider, RootProvider},
    rpc::{client::RpcClient, types::eth::TransactionRequest},
    transports::http::Http,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().try_spawn()?;

    let url = anvil.endpoint().parse().unwrap();
    let http = Http::new(url);
    let provider = RootProvider::<Ethereum, _>::new(RpcClient::new(http, true));

    let accounts = provider.get_accounts().await?;
    let from = accounts[0];
    let to = accounts[1];

    // craft the tx
    let tx = TransactionRequest {
        from: Some(from),
        to: Some(to),
        value: Some(U256::from(1000)),
        ..Default::default()
    };

    let balance_before = provider.get_balance(from, None).await?;
    let nonce_before = provider.get_transaction_count(from, None).await?;

    // broadcast it via the eth_sendTransaction API
    let builder = provider.send_transaction(tx).await?;

    println!("{:#?}", &builder);

    let balance_after = provider.get_balance(from, None).await?;
    let nonce_after = provider.get_transaction_count(from, None).await?;

    assert!(nonce_before < nonce_after);
    assert!(balance_after < balance_before);

    println!("Balance before: {balance_before}");
    println!("Balance after: {balance_after}");

    println!("Nonce before: {nonce_before}");
    println!("Nonce after: {nonce_after}");

    Ok(())
}
