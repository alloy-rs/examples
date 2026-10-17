//! Encode and decode signed Ethereum transaction envelopes with EIP-2718.
//!
//! Always wrap signed transactions in [`TxEnvelope`] before encoding them. The envelope preserves
//! the transaction type byte and handles legacy transactions, which have no type byte.

use alloy::{
    consensus::{
        transaction::SignerRecoverable, SignableTransaction, Signed, TxEip1559, TxEip2930,
        TxEip4844, TxEip7702, TxEnvelope, TxLegacy, TxType,
    },
    eips::{
        eip2718::{Decodable2718, Encodable2718},
        eip2930::AccessList,
        eip7702::Authorization,
    },
    primitives::{address, b256, Address, Signature, TxKind, U256},
    rpc::types::Transaction as RpcTransaction,
    signers::{local::PrivateKeySigner, SignerSync},
};
use eyre::Result;

fn main() -> Result<()> {
    // Fixed test-only keys keep the bytes and hashes reproducible. Never use them on a real chain.
    let signer = PrivateKeySigner::from_bytes(&b256!(
        "0000000000000000000000000000000000000000000000000000000000000001"
    ))?;
    let authorization_signer = PrivateKeySigner::from_bytes(&b256!(
        "0000000000000000000000000000000000000000000000000000000000000002"
    ))?;
    let signer_address = signer.address();
    let to = address!("1111111111111111111111111111111111111111");
    let authorization = Authorization { chain_id: U256::from(1), address: to, nonce: 0 };
    let authorization_signature =
        authorization_signer.sign_hash_sync(&authorization.signature_hash())?;
    let signed_authorization = authorization.into_signed(authorization_signature);
    assert_eq!(signed_authorization.recover_authority()?, authorization_signer.address());

    let envelopes = [
        (
            TxType::Legacy,
            sign(
                TxLegacy {
                    chain_id: Some(1),
                    nonce: 0,
                    gas_price: 20_000_000_000,
                    gas_limit: 21_000,
                    to: TxKind::Call(to),
                    value: U256::from(1),
                    ..Default::default()
                },
                &signer,
            )?,
        ),
        (
            TxType::Eip2930,
            sign(
                TxEip2930 {
                    chain_id: 1,
                    nonce: 1,
                    gas_price: 20_000_000_000,
                    gas_limit: 21_000,
                    to: TxKind::Call(to),
                    value: U256::from(2),
                    access_list: AccessList::default(),
                    ..Default::default()
                },
                &signer,
            )?,
        ),
        (
            TxType::Eip1559,
            sign(
                TxEip1559 {
                    chain_id: 1,
                    nonce: 2,
                    max_fee_per_gas: 20_000_000_000,
                    max_priority_fee_per_gas: 1_000_000_000,
                    gas_limit: 21_000,
                    to: TxKind::Call(to),
                    value: U256::from(3),
                    access_list: AccessList::default(),
                    ..Default::default()
                },
                &signer,
            )?,
        ),
        (
            TxType::Eip4844,
            // This is the execution/block form. Submitting a blob transaction requires a sidecar;
            // see the `send_eip4844_transaction` example for the pooled representation.
            sign(
                TxEip4844 {
                    chain_id: 1,
                    nonce: 3,
                    max_fee_per_gas: 20_000_000_000,
                    max_priority_fee_per_gas: 1_000_000_000,
                    gas_limit: 100_000,
                    to,
                    value: U256::from(4),
                    access_list: AccessList::default(),
                    blob_versioned_hashes: vec![b256!(
                        "0100000000000000000000000000000000000000000000000000000000000000"
                    )],
                    max_fee_per_blob_gas: 1_000_000_000,
                    ..Default::default()
                },
                &signer,
            )?,
        ),
        (
            TxType::Eip7702,
            sign(
                TxEip7702 {
                    chain_id: 1,
                    nonce: 4,
                    max_fee_per_gas: 20_000_000_000,
                    max_priority_fee_per_gas: 1_000_000_000,
                    gas_limit: 50_000,
                    to,
                    value: U256::from(5),
                    access_list: AccessList::default(),
                    authorization_list: vec![signed_authorization],
                    ..Default::default()
                },
                &signer,
            )?,
        ),
    ];

    for (tx_type, envelope) in envelopes {
        round_trip(envelope, tx_type, signer_address)?;
    }

    // Ethereum's concrete JSON-RPC transaction response wraps a `TxEnvelope` with the `from`
    // address reported by the node.
    // `into_inner` removes the RPC-only block metadata without serializing through JSON again.
    let rpc_transaction: RpcTransaction = serde_json::from_str(RPC_TRANSACTION)?;
    let rpc_from = rpc_transaction.inner.signer();
    let reported_hash = *rpc_transaction.inner.tx_hash();
    let envelope: TxEnvelope = rpc_transaction.into_inner();

    // Verify the node-reported sender against the signature. The envelope initially retains the
    // hash reported in JSON, so decode its canonical bytes to independently recompute the hash.
    assert_eq!(envelope.recover_signer()?, rpc_from);
    assert_eq!(*envelope.tx_hash(), reported_hash);
    let encoded = envelope.encoded_2718();
    let decoded = TxEnvelope::decode_2718_exact(&encoded)?;
    assert_eq!(decoded.tx_type(), TxType::Eip1559);
    assert_eq!(decoded.recover_signer()?, rpc_from);
    assert_eq!(*decoded.tx_hash(), reported_hash);

    Ok(())
}

fn sign<T>(transaction: T, signer: &PrivateKeySigner) -> Result<TxEnvelope>
where
    T: SignableTransaction<Signature>,
    Signed<T>: Into<TxEnvelope>,
{
    let signature = signer.sign_hash_sync(&transaction.signature_hash())?;
    Ok(transaction.into_signed(signature).into())
}

fn round_trip(envelope: TxEnvelope, tx_type: TxType, signer: Address) -> Result<()> {
    let tx_hash = *envelope.tx_hash();

    // `encoded_2718` allocates. `encode_2718` appends to a caller-owned buffer.
    let encoded = envelope.encoded_2718();
    let mut reusable_buffer = Vec::with_capacity(envelope.encode_2718_len());
    envelope.encode_2718(&mut reusable_buffer);
    assert_eq!(encoded, reusable_buffer);
    reusable_buffer.clear();
    envelope.encode_2718(&mut reusable_buffer);
    assert_eq!(encoded, reusable_buffer);
    if tx_type == TxType::Legacy {
        assert!(encoded[0] >= 0xc0);
    } else {
        assert_eq!(encoded[0], tx_type as u8);
    }

    // Use the exact decoder for a buffer that must contain one envelope and no trailing bytes.
    let decoded = TxEnvelope::decode_2718_exact(&encoded)?;
    assert_eq!(decoded.tx_type(), tx_type);
    assert_eq!(*decoded.tx_hash(), tx_hash);
    assert_eq!(decoded.recover_signer()?, signer);
    assert_eq!(decoded, envelope);

    Ok(())
}

// Response from `eth_getTransactionByHash` on a local Anvil chain.
const RPC_TRANSACTION: &str = r#"{
  "hash": "0x018b2331d461a4aeedf6a1f9cc37463377578244e6a35216057a8370714e798f",
  "nonce": "0x1",
  "blockHash": "0x6e4e53d1de650d5a5ebed19b38321db369ef1dc357904284ecf4d89b8834969c",
  "blockNumber": "0x2",
  "transactionIndex": "0x0",
  "from": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
  "to": "0x5fbdb2315678afecb367f032d93f642f64180aa3",
  "value": "0x0",
  "gasPrice": "0x3a29f0f8",
  "gas": "0x1c9c380",
  "maxFeePerGas": "0xba43b7400",
  "maxPriorityFeePerGas": "0x5f5e100",
  "input": "0xd09de08a",
  "r": "0xd309309a59a49021281cb6bb41d164c96eab4e50f0c1bd24c03ca336e7bc2bb7",
  "s": "0x28a7f089143d0a1355ebeb2a1b9f0e5ad9eca4303021c1400d61bc23c9ac5319",
  "v": "0x0",
  "yParity": "0x0",
  "chainId": "0x7a69",
  "accessList": [],
  "type": "0x2"
}"#;
