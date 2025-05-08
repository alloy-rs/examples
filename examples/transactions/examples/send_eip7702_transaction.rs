//! Example showing how to send an [EIP-7702](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-7702.md) transaction.

use alloy::{
    eips::eip7702::Authorization,
    network::{TransactionBuilder, TransactionBuilder7702},
    node_bindings::Anvil,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::{local::PrivateKeySigner, SignerSync},
    sol,
};
use eyre::Result;

// Codegen from embedded Solidity code and precompiled bytecode.
// solc v0.8.25 Log.sol --via-ir --optimize --bin
sol!(
    #[allow(missing_docs)]
    #[sol(rpc, bytecode = "6080806040523460135760c9908160188239f35b5f80fdfe6004361015600b575f80fd5b5f3560e01c80637b3ab2d014605f57639ee1a440146027575f80fd5b34605b575f366003190112605b577f2d67bb91f17bca05af6764ab411e86f4ddf757adb89fcec59a7d21c525d417125f80a1005b5f80fd5b34605b575f366003190112605b577fbcdfe0d5b27dd186282e187525415c57ea3077c34efb39148111e4d342e7ab0e5f80a100fea2646970667358221220f6b42b522bc9fb2b4c7d7e611c7c3e995d057ecab7fd7be4179712804c886b4f64736f6c63430008190033")]
    contract Log {
        #[derive(Debug)]
        event Hello();
        event World();

        function emitHello() public {
            emit Hello();
        }

        function emitWorld() public {
            emit World();
        }
    }
);

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node with the Prague hardfork enabled.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().arg("--hardfork").arg("prague").try_spawn()?;

    // Create two users, Alice and Bob.
    // Alice will sign the authorization and Bob will send the transaction.
    let alice: PrivateKeySigner = anvil.keys()[0].clone().into();
    let bob: PrivateKeySigner = anvil.keys()[1].clone().into();

    // Create a provider with the wallet for only Bob (not Alice).
    let rpc_url = anvil.endpoint_url();
    let provider = ProviderBuilder::new().wallet(bob.clone()).connect_http(rpc_url);

    // Deploy the contract Alice will authorize.
    let contract = Log::deploy(&provider).await?;

    // Create an authorization object for Alice to sign.
    let authorization = Authorization {
        chain_id: U256::from(anvil.chain_id()),
        // Reference to the contract that will be set as code for the authority.
        address: *contract.address(),
        nonce: provider.get_transaction_count(alice.address()).await?,
    };

    // Alice signs the authorization.
    let signature = alice.sign_hash_sync(&authorization.signature_hash())?;
    let signed_authorization = authorization.into_signed(signature);

    // Collect the calldata required for the transaction.
    let call = contract.emitHello();
    let emit_hello_calldata = call.calldata().to_owned();

    // Build the transaction.
    let tx = TransactionRequest::default()
        .with_to(alice.address())
        .with_authorization_list(vec![signed_authorization])
        .with_input(emit_hello_calldata);

    // Send the transaction and wait for the broadcast.
    let pending_tx = provider.send_transaction(tx).await?;

    println!("Pending transaction... {}", pending_tx.tx_hash());

    // Wait for the transaction to be included and get the receipt.
    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block {}",
        receipt.block_number.expect("Failed to get block number")
    );

    assert!(receipt.status());
    assert_eq!(receipt.from, bob.address());
    assert_eq!(receipt.to, Some(alice.address()));
    assert_eq!(receipt.inner.logs().len(), 1);
    assert_eq!(receipt.inner.logs()[0].address(), alice.address());

    Ok(())
}
