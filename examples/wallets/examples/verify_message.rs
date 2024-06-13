//! Example of verifying that a message was signed by the provided address.

use alloy::signers::{local::PrivateKeySigner, SignerSync};
use eyre::{Ok, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Instantiate a signer.
    let signer = PrivateKeySigner::random();

    // Sign a message.
    let message = "Some data";
    let signature = signer.sign_message_sync(message.as_bytes())?;

    // Recover the signer from the message.
    let recovered = signature.recover_address_from_msg(message)?;
    assert_eq!(recovered, signer.address());

    Ok(())
}
