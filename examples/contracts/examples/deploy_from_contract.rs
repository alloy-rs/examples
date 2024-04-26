//! Example of deploying a contract from Solidity code to Anvil and interacting with it.

use alloy::{
    network::EthereumSigner, node_bindings::Anvil, primitives::U256, providers::ProviderBuilder,
    signers::wallet::LocalWallet, sol,
};
use eyre::Result;

// Codegen from embedded Solidity code and precompiled bytecode.
sol! {
    #[allow(missing_docs)]
    // solc v0.8.24; solc a.sol --via-ir --optimize --bin
    #[sol(rpc, bytecode="608080604052346100155760d2908161001a8239f35b5f80fdfe60808060405260043610156011575f80fd5b5f3560e01c9081633fb5c1cb1460865781638381f58a14606f575063d09de08a146039575f80fd5b34606b575f366003190112606b575f545f1981146057576001015f55005b634e487b7160e01b5f52601160045260245ffd5b5f80fd5b34606b575f366003190112606b576020905f548152f35b34606b576020366003190112606b576004355f5500fea2646970667358221220bdecd3c1dd631eb40587cafcd6e8297479db76db6a328e18ad1ea5b340852e3864736f6c63430008180033")]
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
    let signer: LocalWallet = anvil.keys()[0].clone().into();

    println!("Anvil running at `{}`", anvil.endpoint());

    // Create a provider with a signer.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(signer))
        .on_http(rpc_url);

    println!("Anvil running at `{}`", anvil.endpoint());

    // Deploy the contract.
    let contract = Counter::deploy(&provider).await?;

    println!("Deployed contract at address: {:?}", contract.address());

    let builder = contract.setNumber(U256::from(42));
    let receipt = builder.send().await?.get_receipt().await?;

    println!("Set number to 42: {:?}", receipt.transaction_hash);

    // Increment the number to 43.
    let builder = contract.increment();
    let receipt = builder.send().await?.get_receipt().await?;

    println!("Incremented number: {:?}", receipt.transaction_hash);

    // Retrieve the number, which should be 43.
    let Counter::numberReturn { _0 } = contract.number().call().await?;

    println!("Retrieved number: {:?}", _0.to_string());

    Ok(())
}
