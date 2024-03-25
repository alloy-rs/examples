//! Example of how to transfer ETH from one account to another.

use alloy::{
    network::Ethereum,
    node_bindings::Anvil,
    primitives::{address, Address, U256},
    providers::{HttpProvider, Provider},
    rpc::types::eth::TransactionRequest,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().fork("https://eth.merkle.io").spawn();
    let url = anvil.endpoint().parse().unwrap();
    let provider = HttpProvider::<Ethereum>::new_http(url);

    let from = anvil.addresses()[0];
    // Transfer 1ETH from 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266 to Address::ZERO
    let tx = TransactionRequest::default().from(from).value(U256::from(1)).to(Some(Address::ZERO));

    let tx = provider.send_transaction(tx).await?;
    let hash = tx.tx_hash();
    println!("Pending transaction hash: {}", hash);

    let transaction = provider.get_transaction_by_hash(hash.to_owned()).await?;

    assert_eq!(transaction.from, address!("f39fd6e51aad88f6f4ce6ab8827279cfffb92266"));
    assert_eq!(transaction.to, Some(Address::ZERO));
    assert_eq!(transaction.value, U256::from(1));

    Ok(())
}
