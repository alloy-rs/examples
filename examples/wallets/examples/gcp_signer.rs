//! Example of how to use GCP Ethereum Signer.
use alloy::signers::gcp::{GcpKeyRingRef, GcpSigner, KeySpecifier};
use alloy::signers::Signer;
use gcloud_sdk::{
    google::cloud::kms::v1::key_management_service_client::KeyManagementServiceClient, GoogleApi,
};

#[tokio::main]
async fn main() {
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
    .expect("Failed to create GCP KMS Client");

    let key_version = 1;

    let specifier = KeySpecifier::new(keyring, &key_name, key_version);
    let signer = GcpSigner::new(client, specifier, Some(key_version)).await.expect("get_key");
    let message = vec![0, 1, 2, 3];
    let sig = signer.sign_message(&message).await.unwrap();
    assert_eq!(sig.recover_address_from_msg(message).unwrap(), signer.address());
}
