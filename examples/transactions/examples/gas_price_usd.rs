//! Example of how to get the gas price in USD using the Chainlink ETH/USD feed.

use alloy::{
    network::TransactionBuilder,
    primitives::{address, utils::format_units, Address, Bytes, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
};
use eyre::Result;
use std::str::FromStr;

const ETH_USD_FEED: Address = address!("5f4eC3Df9cbd43714FE2740f5E3616155c5b8419");
const ETH_USD_FEED_DECIMALS: u8 = 8;
const ETH_DECIMALS: u32 = 18;

// Codegen from excerpt of Chainlink Aggregator interface.
// See: https://etherscan.io/address/0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419#code
sol!(
    #[allow(missing_docs)]
    function latestAnswer() external view returns (int256);
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let rpc_url = "https://eth.merkle.io";
    let provider = ProviderBuilder::new().on_anvil_with_config(|anvil| anvil.fork(rpc_url));

    // Create a call to get the latest answer from the Chainlink ETH/USD feed.
    let call = latestAnswerCall {}.abi_encode();
    let input = Bytes::from(call);

    // Call the Chainlink ETH/USD feed contract.
    let tx = TransactionRequest::default().with_to(ETH_USD_FEED).with_input(input);

    let response = provider.call(tx).await?;
    let result = U256::from_str(&response.to_string())?;

    // Get the gas price of the network.
    let wei_per_gas = provider.get_gas_price().await?;

    // Convert the gas price to Gwei and USD.
    let gwei = format_units(wei_per_gas, "gwei")?.parse::<f64>()?;
    let usd = get_usd_value(wei_per_gas, result)?;

    println!("Gas price in Gwei: {gwei}");
    println!("Gas price in USD: {usd}");

    Ok(())
}

fn get_usd_value(amount: u128, price_usd: U256) -> Result<f64> {
    let base = U256::from(10).pow(U256::from(ETH_DECIMALS));
    let value = U256::from(amount) * price_usd / base;
    let formatted = format_units(value, ETH_USD_FEED_DECIMALS)?.parse::<f64>()?;

    Ok(formatted)
}
