//! Example of encoding and decoding raw transactions.

use alloy::primitives::keccak256;
use alloy::primitives::private::alloy_rlp::Decodable;
use alloy::primitives::private::alloy_rlp::Encodable;
use alloy::primitives::FixedBytes;
use alloy::primitives::TxKind;
use alloy::primitives::U256;
use alloy::providers::WalletProvider;
use alloy::{
    consensus::TxEnvelope,
    consensus::{SignableTransaction, TxEip1559},
    hex,
    network::{EthereumWallet, TransactionBuilder},
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
};
use eyre::Result;

fn build_unsigned_tx(chain_id: u64, to_address: Address) -> TxEip1559 {
    TxEip1559 {
        chain_id,
        nonce: 0,
        gas_limit: 21_000,
        max_fee_per_gas: 20_000_000_000,
        max_priority_fee_per_gas: 1_000_000_000,
        to: TxKind::Call(to_address), // Change this to `TxKind::Create` if you'd like to deploy a contract instead
        value: U256::from(100),
        ..Default::default()
    }
}

fn unsigned_tx_to_bytes(tx: TxEip1559) -> Vec<u8> {
    tx.encoded_for_signing() // To use this, have to import "alloy::primitives::private::alloy_rlp::Encodable"
}

fn bytes_to_unsigned_tx(bytes: Vec<u8>) -> TxEip1559 {
    let mut slice = &bytes.as_slice()[1..];
    TxEip1559::decode(&mut slice).unwrap() // To use this, have to import "alloy::primitives::private::alloy_rlp::Decodable"
}

async fn sign_tx(tx: TxEip1559, wallet: EthereumWallet) -> TxEnvelope {
    let tx_request: TransactionRequest = tx.into();

    tx_request.build(&wallet).await.unwrap()
}

fn signed_tx_to_bytes(signed_tx: TxEnvelope) -> Vec<u8> {
    let mut encoded = Vec::new();
    signed_tx.encode(&mut encoded);
    let encoded = &encoded[2..];
    encoded.into()
}

fn bytes_to_signed_tx(bytes: Vec<u8>) -> TxEnvelope {
    let mut slice = bytes.as_slice();
    TxEnvelope::decode(&mut slice).unwrap()
}

fn get_tx_hash_from_signed_tx_bytes(signed_tx_bytes: Vec<u8>) -> FixedBytes<32> {
    format!("0x{}", hex::encode(keccak256(signed_tx_bytes))).parse::<FixedBytes<32>>().unwrap()
}

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let provider = ProviderBuilder::new().on_anvil_with_wallet();

    // Create two users, Alice and Bob.
    let accounts = provider.get_accounts().await?;
    let alice = accounts[0];
    let bob = accounts[1];

    // 1. Build a transaction to send 100 wei from Alice to Bob.
    let tx: TxEip1559 = build_unsigned_tx(provider.get_chain_id().await?, bob);

    // 2. Encode the unsigned transaction to rlp bytes.
    let unsigned_tx_bytes = unsigned_tx_to_bytes(tx.clone());
    assert_eq!(tx, bytes_to_unsigned_tx(unsigned_tx_bytes));

    // 3. Sign the transaction using the wallet.
    let signed_tx = sign_tx(tx, provider.wallet().clone()).await;

    // 4. Encode the signed transaction to rlp bytes.
    let signed_tx_bytes = signed_tx_to_bytes(signed_tx.clone());
    assert_eq!(signed_tx, bytes_to_signed_tx(signed_tx_bytes.clone()));

    // 5. Send the raw transaction and retrieve the transaction receipt.
    let receipt = provider.send_tx_envelope(signed_tx).await?.get_receipt().await?;
    assert_eq!(receipt.from, alice);
    assert_eq!(receipt.to, Some(bob));

    // 6. Comapre the transaction hash from the signed transaction with the receipt's transaction hash.
    let tx_hash = get_tx_hash_from_signed_tx_bytes(signed_tx_bytes.clone());
    assert_eq!(tx_hash, receipt.transaction_hash);

    Ok(())
}
