//! Example for encoding calldata via sol!

use std::str::FromStr;

use alloy::{
    hex,
    primitives::{Address, U256},
    sol,
    sol_types::SolCall,
};

sol!(
    #[allow(missing_docs)]
    function swapExactTokensForTokens(
        uint256 amountIn,
        uint256 amountOutMin,
        address[] calldata path,
        address to,
        uint256 deadline
      ) external returns (uint256[] memory amounts);
);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let amount_in = U256::from(1000000000000000000u128); // 1 token
    let amount_out_min = U256::from(50000000000000000u128); // 0.99 tokens (1% slippage)
    let token_in = Address::from_str("0x6B175474E89094C44Da98b954EedeAC495271d0F")?; // DAI
    let weth = Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")?; // WETH
    let token_out = Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")?; // USDC
    let path = vec![token_in, weth, token_out];
    let to_address = Address::from_str("0x742d35Cc6634C0532925a3b844Bc454e4438f44e")?;
    let deadline = U256::from(1690000000u64); // random timestamp

    let swap =
        swapExactTokensForTokensCall::new((amount_in, amount_out_min, path, to_address, deadline));

    let encoded = swapExactTokensForTokensCall::abi_encode(&swap);

    println!("Encoded: 0x{}", hex::encode(&encoded));

    Ok(())
}
