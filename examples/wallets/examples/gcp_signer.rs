//! Example of how to use the GCP KMS Signer.
use alloy::signers::{
    gcp::{GcpKeyRingRef, GcpSigner, KeySpecifier},
    Signer,
};
use eyre::{Ok, Result};
use gcloud_sdk::{
    google::cloud::kms::v1::key_management_service_client::KeyManagementServiceClient, GoogleApi,
};

#[tokio::main]
async fn main() -> Result<()> {
    // environment variable, preferably to be set in a dotenv file
    let project_id = "GOOGLE_PROJECT_ID";
    let location = "GOOGLE_LOCATION";
    let keyring = "GOOGLE_KEYRING";
    let key_name = "GOOGLE_KEY_NAME";

    let keyring = GcpKeyRingRef::new(&project_id, &location, &keyring);
    let client = GoogleApi::from_function(
        KeyManagementServiceClient::new,
        "https://cloudkms.googleapis.com",
        None,
    )
    .await
    .unwrap();

    let key_version = 1;

    let specifier = KeySpecifier::new(keyring, &key_name, key_version);
    let signer = GcpSigner::new(client, specifier, Some(key_version)).await.unwrap();
    let message = "Hello, world!";
    let signature = signer.sign_message(message.as_bytes()).await.unwrap();
    assert_eq!(signature.recover_address_from_msg(message)?, signer.address());
    Ok(())
}
