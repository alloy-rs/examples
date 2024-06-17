//! Example of deploying a contract to Anvil and interacting with it.

use alloy::{
    network::EthereumWallet, node_bindings::Anvil, primitives::U256, providers::ProviderBuilder,
    signers::local::PrivateKeySigner, sol,
};
use eyre::Result;

// Codegen from embedded Solidity code and precompiled bytecode.
sol! {
    #[allow(missing_docs)]
    // solc v0.8.26; solc a.sol --via-ir --optimize --bin
    #[sol(rpc, bytecode="6080806040523460135760df908160198239f35b600080fdfe6080806040526004361015601257600080fd5b60003560e01c9081633fb5c1cb1460925781638381f58a146079575063d09de08a14603c57600080fd5b3460745760003660031901126074576000546000198114605e57600101600055005b634e487b7160e01b600052601160045260246000fd5b600080fd5b3460745760003660031901126074576020906000548152f35b34607457602036600319011260745760043560005500fea2646970667358221220e978270883b7baed10810c4079c941512e93a7ba1cd1108c781d4bc738d9090564736f6c634300081a0033")]
    contract Counter {
        uint256 public number;

        function setNumber(uint256 newNumber) public {
            number = newNumber;
        }

        function increment() public {
            number++;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().try_spawn()?;

    // Set up signer from the first default Anvil account (Alice).
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::from(signer);

    // Create a provider with the wallet.
    let rpc_url = anvil.endpoint().parse()?;
    let provider =
        ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(rpc_url);

    println!("Anvil running at `{}`", anvil.endpoint());

    // Deploy the contract.
    let contract = Counter::deploy(&provider).await?;

    println!("Deployed contract at address: {}", contract.address());

    // Set the number to 42.
    let builder = contract.setNumber(U256::from(42));
    let tx_hash = builder.send().await?.watch().await?;

    println!("Set number to 42: {tx_hash}");

    // Increment the number to 43.
    let builder = contract.increment();
    let tx_hash = builder.send().await?.watch().await?;

    println!("Incremented number: {tx_hash}");

    // Retrieve the number, which should be 43.
    let builder = contract.number();
    let number = builder.call().await?.number.to_string();

    println!("Retrieved number: {number}");

    Ok(())
}
