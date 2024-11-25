//! Example depicting how to make a Batch RPC request using the HTTP provider.

use alloy::{
    node_bindings::Anvil,
    primitives::{address, U128, U64},
    rpc::client::ClientBuilder,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().spawn();

    // Swap this out with a RPC_URL provider that supports JSON-RPC batch requests. e.g. https://eth.merkle.io
    let rpc_url = anvil.endpoint_url();

    // Create a HTTP transport.
    let client = ClientBuilder::default().http(rpc_url);

    // Instantiate a batch.
    let mut batch = client.new_batch();

    // Add calls to the batch.
    let block_number_fut =
        batch.add_call("eth_blockNumber", &())?.map_resp(|resp: U64| resp.to::<u64>());

    let gas_price_fut =
        batch.add_call("eth_gasPrice", &())?.map_resp(|resp: U128| resp.to::<u128>());

    let vitalik = address!("d8da6bf26964af9d7eed9e03e53415d37aa96045");

    let vitalik_nonce_fut = batch
        .add_call("eth_getTransactionCount", &(vitalik, "latest"))? // Vitalik's nonce at BlockId::Latest
        .map_resp(|resp: U128| resp.to::<u128>());

    // Send the batch request.
    batch.send().await?;

    // Get the results.
    let (latest_block, gas_price, vitalik_nonce) =
        tokio::try_join!(block_number_fut, gas_price_fut, vitalik_nonce_fut)?;

    println!("Latest block number: {latest_block}");
    println!("Gas price: {gas_price}");
    println!("Vitalik's nonce: {vitalik_nonce}");

    Ok(())
}
