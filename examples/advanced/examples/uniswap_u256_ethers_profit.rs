//! Uniswap V2 Arbitrage Profit Calculation using ethers-rs U256

use eyre::Result;
use helpers::ethers::{
    display_token, get_amount_in, get_amount_out, get_sushi_pair, get_uniswap_pair,
};

fn main() -> Result<()> {
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
    println!("Ethers-rs U256");
    println!("WETH amount in {}", display_token(amount_in));
    println!("WETH profit: {}", display_token(profit));

    Ok(())
}
