//! Example of signing a message with a wallet.

use alloy::signers::{wallet::LocalWallet, Signer};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up a random signer.
    let signer = LocalWallet::random();

    // Optionally, the wallet's chain id can be set, in order to use EIP-155
    // replay protection with different chains.
    let signer = signer.with_chain_id(Some(1337));

    // The message to sign.
    let message = b"hello";

    // Sign the message asynchronously with the signer.
    let signature = signer.sign_message(message).await?;

    println!("Signature produced by {:?}: {:?}", signer.address(), signature);
    println!(
        "Signature recovered address: {:?}",
        signature.recover_address_from_msg(&message[..])?
    );

    Ok(())
}
