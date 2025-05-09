//! Example for static encoding calldata via `sol!`.

use std::str::FromStr;

use alloy::{
    hex,
    primitives::{Address, U256},
    sol,
    sol_types::SolCall,
};

// Using UniswapV2 `swapExactTokensForTokens()` method for this example.
// See: https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexacttokensfortokens
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
    // Swap 1 DAI for 1 USDC with a slippage tolerance of 1%.
    let amount_in = U256::from(1000000000000000000u128); // 1 token
    let amount_out_min = U256::from(9900000000000000000u128); // 0.99 tokens (1% slippage)

    // Construct path DAI --> WETH --> USDC.
    let token_in = Address::from_str("0x6B175474E89094C44Da98b954EedeAC495271d0F")?; // DAI
    let weth = Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")?; // WETH
    let token_out = Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")?; // USDC
    let path = vec![token_in, weth, token_out];

    // Recipient of the output tokens.
    let to = Address::from_str("0x742d35Cc6634C0532925a3b844Bc454e4438f44e")?;

    // Unix timestamp after which the transaction will revert.
    let deadline = U256::from(1690000000u64); // Random timestamp

    let swap_data =
        swapExactTokensForTokensCall::new((amount_in, amount_out_min, path, to, deadline));

    let encoded = hex::encode(swapExactTokensForTokensCall::abi_encode(&swap_data));

    println!("Encoded: 0x{encoded}");

    Ok(())
}
