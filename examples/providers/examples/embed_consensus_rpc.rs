//! This example demonstrates how alloy's RPC types and consensus types are tied together.
//!
//! Consensus types are used in Ethereum execution layer consensus and communication. These include
//! transactions, headers, blocks, [EIP-2718](https://eips.ethereum.org/EIPS/eip-2718) envelopes, [EIP-2930](https://eips.ethereum.org/EIPS/eip-2930), [EIP-4844](https://eips.ethereum.org/EIPS/eip-4844), and more.
//!
//! The RPC types are used to communicate with Ethereum nodes.
//!
//! In the case of alloy, the consensus types are embedded into the RPC types unlocking a ton of
//! simplications across these two categories of types and also preventing accidental divergence
//! between the two.
//! This has been achieved without altering the resultant serialized JSON-RPC representations.
//!
//! See:
//!
//! - [Embed consensus `Header` into RPC type](https://github.com/alloy-rs/alloy/pull/1573)
//! - [Embed `TxEnvelope` into RPC `Transaction`](https://github.com/alloy-rs/alloy/pull/1460)
//! - [Embed consenus `Log` and `Receipt` into respective RPC types](https://github.com/alloy-rs/alloy/pull/396)

use alloy::{
    eips::BlockId,
    primitives::b256,
    providers::{Provider, ProviderBuilder},
};
use eyre::OptionExt;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = ProviderBuilder::new().connect("https://reth-ethereum.ithaca.xyz/rpc").await?;

    // Get the latest block from the RPC.
    let block = provider.get_block(BlockId::latest()).await?.ok_or_eyre("Block not found")?;
    // The immediate type returned is the RPC `Block` type.
    assert!(matches!(block, alloy::rpc::types::Block { .. }));
    // This rpc block type contains the RPC `Header` type which encapsulates the consensus `Header`
    // type.
    assert!(matches!(block.header.inner, alloy::consensus::Header { .. }));
    // One can use the `into_consensus method` to get the consensus representation of the block.
    let consensus = block.into_consensus();
    assert!(matches!(consensus, alloy::consensus::Block { .. }));

    // Similarly, the RPC `Transaction` and `TransactionReceipt` types encapsulate their
    // corresponding consensus types.

    // Get a transaction by hash
    // <https://etherscan.io/tx/0x5b470467985bfd34f18979b5438ffce4f2a309a32bcc857fcbf48c4e4253ce16>
    let tx_hash = b256!("0x5b470467985bfd34f18979b5438ffce4f2a309a32bcc857fcbf48c4e4253ce16");
    let tx =
        provider.get_transaction_by_hash(tx_hash).await?.ok_or_eyre("Transaction not found")?;
    assert!(matches!(tx, alloy::rpc::types::Transaction { .. }));
    // The RPC `Transaction` type contains the consensus `EthereumTxEnvelope` type.
    assert!(matches!(tx.into_inner(), alloy::consensus::EthereumTxEnvelope::Eip1559(_)));

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await?
        .ok_or_eyre("Transaction receipt not found")?;
    assert!(matches!(receipt, alloy::rpc::types::TransactionReceipt { .. }));
    // The `TransactionReceipt` type contains the consensus `ReceiptEnvelope` type.
    assert!(matches!(receipt.inner, alloy::consensus::ReceiptEnvelope::Eip1559(_)));

    Ok(())
}
