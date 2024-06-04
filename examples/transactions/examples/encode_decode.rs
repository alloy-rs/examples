//! Encode and decode transactions
use alloy::{
    consensus::{SignableTransaction, TxEip1559},
    eips::eip2930::AccessList,
    primitives::{address, b256, hex, Signature, TxKind, U256},
};
use eyre::Result;
#[tokio::main]
async fn main() -> Result<()> {
    // EIP1559 transaction
    let signer = address!("DD6B8b3dC6B7AD97db52F08a275FF4483e024CEa");
    let hash = b256!("0ec0b6a2df4d87424e5f6ad2a654e27aaeb7dac20ae9e8385cc09087ad532ee0");
    let tx =  TxEip1559 {
        chain_id: 1,
        nonce: 0x42,
        gas_limit: 44386,
        to: TxKind::Call( address!("6069a6c32cf691f5982febae4faf8a6f3ab2f0f6")),
        value: U256::from(0_u64),
        input:  hex!("a22cb4650000000000000000000000005eee75727d804a2b13038928d36f8b188945a57a0000000000000000000000000000000000000000000000000000000000000000").into(),
        max_fee_per_gas: 0x4a817c800,
        max_priority_fee_per_gas: 0x3b9aca00,
        access_list: AccessList::default(),
    };

    let sig = Signature::from_scalars_and_parity(
        b256!("840cfc572845f5786e702984c2a582528cad4b49b2a10b9db1be7fca90058565"),
        b256!("25e7109ceb98168d95b09b18bbf6b685130e0562f233877d492b94eee0c5b6d1"),
        false,
    )
    .unwrap();

    let signed_tx = tx.into_signed(sig);

    // Match hash
    assert_eq!(*signed_tx.hash(), hash);

    let recovered_signer = signed_tx.recover_signer().unwrap();
    assert_eq!(recovered_signer, signer);

    Ok(())
}
