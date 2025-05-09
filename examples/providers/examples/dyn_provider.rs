//! Demonstrates how to obtain a `DynProvider` from a Provider.

use alloy::{
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol,
};

// Codegen from embedded Solidity code and precompiled bytecode.
sol! {
    #[allow(missing_docs)]
    // solc v0.8.26; solc Counter.sol --via-ir --optimize --bin
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
async fn main() -> eyre::Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().spawn();

    let signer_pk: PrivateKeySigner =
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;

    let from = signer_pk.address();

    // Provider with verbose types.
    let regular_provider =
        ProviderBuilder::new().wallet(signer_pk).connect(anvil.endpoint().as_str()).await?;

    // One can use the erased method to obtain a DynProvider from a Provider.
    let dyn_provider = regular_provider.erased();

    // Note that the fillers set while building provider are still available, only the types have
    // been erased OR boxed under the hood.
    // This enables us to use the DynProvider as one would use a regular Provider with verbose
    // types.
    let counter = Counter::deploy(&dyn_provider).await?;

    println!("Counter deployed at {}", counter.address());

    // Sends a transaction with required properties such as gas, nonce, from filled.
    let incr = counter.increment().send().await?;
    let receipt = incr.get_receipt().await?;
    assert_eq!(receipt.from, from);

    let number = counter.number().call().await?;

    println!("New number: {number}");

    Ok(())
}
