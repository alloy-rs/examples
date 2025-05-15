//! Simple arbitrage profit calculator for WETH/DAI pools
//! Reads the balaces of the Uniswap V2 and `Sushiswap` pools and calculates a basic arb
//! opportunity.

mod helpers;
use crate::helpers::{get_amount_in, get_amount_out, get_sushi_pair, get_uniswap_pair};
use alloy::primitives::utils::format_units;
use eyre::Result;

fn main() -> Result<()> {
    // Get the pool contract interfaces
    let uniswap_pair = get_uniswap_pair();
    let sushi_pair = get_sushi_pair();

    let amount_in = get_amount_in(
        uniswap_pair.reserve0,
        uniswap_pair.reserve1,
        false,
        sushi_pair.reserve0,
        sushi_pair.reserve1,
    );

    let dai_amount_out = get_amount_out(uniswap_pair.reserve1, uniswap_pair.reserve0, amount_in);
    let weth_amount_out = get_amount_out(sushi_pair.reserve0, sushi_pair.reserve1, dai_amount_out);

    if weth_amount_out < amount_in {
        println!("No profit detected");
        return Ok(());
    }

    let profit = weth_amount_out - amount_in;
    println!("Alloy U256");
    println!("WETH amount in {}", format_units(amount_in, 18).unwrap());
    println!("WETH profit: {}", format_units(profit, 18).unwrap());

    Ok(())
}
