//! Encode and decode Ethereum receipt envelopes with EIP-2718.
//!
//! Receipts use the same legacy-versus-typed envelope distinction as transactions.

use alloy::{
    consensus::{Receipt, ReceiptEnvelope, TxType},
    eips::eip2718::{Decodable2718, Encodable2718},
    primitives::Log,
};
use eyre::Result;

fn main() -> Result<()> {
    let receipt =
        Receipt::<Log> { status: true.into(), cumulative_gas_used: 21_000, logs: Vec::new() }
            .with_bloom();

    let envelopes = [
        (TxType::Legacy, ReceiptEnvelope::Legacy(receipt.clone())),
        (TxType::Eip2930, ReceiptEnvelope::Eip2930(receipt.clone())),
        (TxType::Eip1559, ReceiptEnvelope::Eip1559(receipt.clone())),
        (TxType::Eip4844, ReceiptEnvelope::Eip4844(receipt.clone())),
        (TxType::Eip7702, ReceiptEnvelope::Eip7702(receipt)),
    ];

    for (tx_type, envelope) in envelopes {
        let encoded = envelope.encoded_2718();
        let decoded = ReceiptEnvelope::decode_2718_exact(&encoded)?;

        assert_eq!(decoded.tx_type(), tx_type);
        assert_eq!(decoded, envelope);
        assert!(decoded.is_success());
        assert_eq!(decoded.cumulative_gas_used(), 21_000);
    }

    Ok(())
}
