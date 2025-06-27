//! Example showing how to use the GCP KMS signer.

use alloy::signers::{
    gcp::{GcpKeyRingRef, GcpSigner, KeySpecifier},
    Signer,
};
use alloy_chains::NamedChain;
use eyre::Result;
use gcloud_sdk::{
    google::cloud::kms::v1::key_management_service_client::KeyManagementServiceClient, GoogleApi,
};

#[tokio::main]
async fn main() -> Result<()> {
    let project_id =
        std::env::var("GOOGLE_PROJECT_ID").expect("GOOGLE_PROJECT_ID not set in .env file");
    let location = std::env::var("GOOGLE_LOCATION").expect("GOOGLE_LOCATION not set in .env file");
    let keyring = std::env::var("GOOGLE_KEYRING").expect("GOOGLE_KEYRING not set in .env file");
    let key_name = std::env::var("GOOGLE_KEY_NAME").expect("GOOGLE_KEY_NAME not set in .env file");

    let keyring = GcpKeyRingRef::new(&project_id, &location, &keyring);
    let client = GoogleApi::from_function(
        KeyManagementServiceClient::new,
        "https://cloudkms.googleapis.com",
        None,
    )
    .await?;

    let key_version = 1;
    let specifier = KeySpecifier::new(keyring, &key_name, key_version);

    let chain = NamedChain::Mainnet;
    let signer = GcpSigner::new(client, specifier, Some(chain.into())).await?;

    let message = "Hello, world!";
    let signature = signer.sign_message(message.as_bytes()).await?;

    assert_eq!(signature.recover_address_from_msg(message)?, signer.address());

    Ok(())
}
