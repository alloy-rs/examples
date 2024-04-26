//! Example showing how to use the AWS KMS signer.

use alloy::signers::{aws::AwsSigner, Signer};
use aws_config::BehaviorVersion;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let Ok(key_id) = std::env::var("AWS_KEY_ID") else {
        return Ok(());
    };

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_kms::Client::new(&config);
    let signer = AwsSigner::new(client, key_id, Some(1)).await?;

    let message = "Hello, world!";
    let signature = signer.sign_message(message.as_bytes()).await.unwrap();

    assert_eq!(signature.recover_address_from_msg(message)?, signer.address());

    Ok(())
}
