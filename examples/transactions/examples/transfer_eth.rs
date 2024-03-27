//! Example of how to transfer ETH from one account to another.

use alloy::{
    network::Ethereum,
    node_bindings::Anvil,
    primitives::U256,
    providers::{HttpProvider, Provider},
    rpc::types::eth::TransactionRequest,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH
    let anvil = Anvil::new().fork("https://eth.merkle.io").try_spawn()?;

    // Create a provider.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = HttpProvider::<Ethereum>::new_http(rpc_url);

    // Create two users, Alice and Bob.
    let alice = anvil.addresses()[0];
    let bob = anvil.addresses()[1];

    // Transfer 1 wei from Alice to Bob.
    let tx = TransactionRequest::default().from(alice).value(U256::from(1)).to(Some(bob));
    let pending_tx = provider.send_transaction(tx).await?;
    let hash = pending_tx.tx_hash();

    println!("Pending transaction hash: {}", hash);

    let transaction = provider.get_transaction_by_hash(hash.to_owned()).await?;

    assert_eq!(transaction.from, alice);
    assert_eq!(transaction.to, Some(bob));
    assert_eq!(transaction.value, U256::from(1));

    Ok(())
}
