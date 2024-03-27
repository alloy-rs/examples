//! Example of how to get the gas price in USD using the Chainlink ETH/USD feed.

use alloy::{
    network::Ethereum,
    node_bindings::Anvil,
    primitives::{address, utils::format_units, Address, Bytes, U256},
    providers::{HttpProvider, Provider},
    rpc::types::eth::TransactionRequest,
    sol,
    sol_types::SolCall,
};
use eyre::Result;
use std::str::FromStr;

const ETH_USD_FEED: Address = address!("5f4eC3Df9cbd43714FE2740f5E3616155c5b8419");
const ETH_USD_FEED_DECIMALS: u8 = 8;
const ETH_DECIMALS: u32 = 18;

sol!(
    #[derive(Debug)]
    function latestAnswer() external view returns (int256);
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH
    let anvil = Anvil::new().fork("https://eth.merkle.io").try_spawn()?;

    // Create a provider.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = HttpProvider::<Ethereum>::new_http(rpc_url);

    let call = latestAnswerCall {}.abi_encode();
    let input = Bytes::from(call);
    let tx = TransactionRequest::default().to(Some(ETH_USD_FEED)).input(Some(input).into());
    let response = provider.call(&tx, None).await?;
    let result = U256::from_str(response.to_string().as_str())?;

    let wei_per_gas = provider.get_gas_price().await?;
    let gwei = format_units(wei_per_gas, "gwei")?.parse::<f64>()?;
    let usd = get_usd_value(wei_per_gas, result)?;

    println!("Gas price in Gwei: {}", gwei);
    println!("Gas price in USD: {}", usd);

    Ok(())
}

fn get_usd_value(amount: U256, price_usd: U256) -> Result<f64> {
    let base: U256 = U256::from(10).pow(U256::from(ETH_DECIMALS));
    let value: U256 = amount * price_usd / base;
    let formatted = format_units(value, ETH_USD_FEED_DECIMALS)?.parse::<f64>()?;

    Ok(formatted)
}
