//! Example of how to decode the input of a transaction.

use alloy::{primitives::hex, sol, sol_types::SolCall};
use eyre::Result;

// Codegen from excerpt of Uniswap V2 Router interface.
// See: https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02
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

#[tokio::main]
async fn main() -> Result<()> {
    println!("Decoding https://etherscan.io/tx/0xd1b449d8b1552156957309bffb988924569de34fbf21b51e7af31070cc80fe9a");

    let input = "0x38ed173900000000000000000000000000000000000000000001a717cc0a3e4f84c00000000000000000000000000000000000000000000000000000000000000283568400000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000201f129111c60401630932d9f9811bd5b5fff34e000000000000000000000000000000000000000000000000000000006227723d000000000000000000000000000000000000000000000000000000000000000200000000000000000000000095ad61b0a150d79219dcf64e1e6cc01f0b64c4ce000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec7";
    let input = hex::decode(input)?;

    // Decode the input using the generated `swapExactTokensForTokens` bindings.
    let decoded = swapExactTokensForTokensCall::abi_decode(&input);

    match decoded {
        Ok(decoded) => {
            let path = decoded.path;

            println!(
                "Swap {} of token {} to {} of token {}",
                decoded.amountIn,
                path.first().expect("Path is empty"),
                decoded.amountOutMin,
                path.last().expect("Path is empty")
            );
        }
        Err(e) => {
            println!("Error decoding input: {e:?}");
        }
    }

    Ok(())
}
