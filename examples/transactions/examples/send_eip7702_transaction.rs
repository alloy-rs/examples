//! This example demonstrates how to send an EIP7702 transaction.
use alloy::{
    consensus::{SignableTransaction, TxEip7702},
    eips::eip7702::Authorization,
    network::TxSignerSync,
    node_bindings::Anvil,
    primitives::{TxKind, U256},
    providers::{Provider, ProviderBuilder},
    signers::{local::LocalSigner, SignerSync},
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
    let anvil = Anvil::new().arg("--hardfork").arg("prague").try_spawn()?;

    let authority = LocalSigner::from_signing_key(anvil.keys()[0].clone().into()); // 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266
    let sender = LocalSigner::from_signing_key(anvil.keys()[1].clone().into());
    let provider = ProviderBuilder::new().on_http(anvil.endpoint_url());

    let contract = Log::deploy(&provider).await?;

    let auth_7702 = Authorization {
        chain_id: U256::from(31337),
        address: *contract.address(), /* Reference to the contract that will be set as code for
                                       * the authority */
        nonce: provider.get_transaction_count(authority.address()).await?,
    };

    // Sign the authorization
    let sig = authority.sign_hash_sync(&auth_7702.signature_hash())?;
    let auth = auth_7702.into_signed(sig);

    // Collect the calldata required for the tx
    let call = contract.emitHello();
    let emit_hello_calldata = call.calldata().to_owned();

    // Estimate the EIP1559 fees
    let eip1559_est = provider.estimate_eip1559_fees(None).await?;

    // Build the transaction
    let mut tx = TxEip7702 {
        to: TxKind::Call(authority.address()),
        authorization_list: vec![auth],
        input: emit_hello_calldata.to_owned(),
        nonce: provider.get_transaction_count(sender.address()).await?,
        chain_id: 31337,
        gas_limit: 1000000,
        max_fee_per_gas: eip1559_est.max_fee_per_gas,
        max_priority_fee_per_gas: eip1559_est.max_priority_fee_per_gas,
        ..Default::default()
    };

    // Sign and Encode the transaction
    let sig = sender.sign_transaction_sync(&mut tx)?;
    let tx = tx.into_signed(sig);
    let mut encoded = Vec::new();
    tx.tx().encode_with_signature(tx.signature(), &mut encoded, false);
    let receipt = provider.send_raw_transaction(&encoded).await?.get_receipt().await?;

    assert!(receipt.status());
    assert_eq!(receipt.inner.logs().len(), 1);
    assert_eq!(receipt.inner.logs()[0].address(), authority.address());

    Ok(())
}
