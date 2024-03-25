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
const ETH_DECIMALS: u32 = 18;

sol!(
    #[derive(Debug)]
    function latestAnswer() external view returns (int256);
);

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().fork("https://eth.merkle.io").spawn();
    let url = anvil.endpoint().parse().unwrap();
    let provider = HttpProvider::<Ethereum>::new_http(url);

    let call = latestAnswerCall {}.abi_encode();
    let input = Bytes::from(call);

    let tx = TransactionRequest::default().to(Some(ETH_USD_FEED)).input(Some(input).into());

    let res = provider.call(&tx, None).await?;

    let u = U256::from_str(res.to_string().as_str());

    let wei_per_gas = provider.get_gas_price().await?;

    let gwei = format_units(wei_per_gas, "gwei")?.parse::<f64>()?;

    let usd = usd_value(wei_per_gas, u.unwrap())?;

    println!("Gas price in Gwei: {}", gwei);
    println!("Gas price in USD: {}", usd);

    Ok(())
}

fn usd_value(amount: U256, price_usd: U256) -> Result<f64> {
    let base: U256 = U256::from(10).pow(U256::from(ETH_DECIMALS));
    let value: U256 = amount * price_usd / base;
    let usd_price_decimals: u8 = 8;
    let f: String = format_units(value, usd_price_decimals)?;
    Ok(f.parse::<f64>()?)
}
