//! Example of creating a keystore file from a private key and password, and then reading it back.

use alloy::{primitives::hex, signers::wallet::Wallet};
use eyre::Result;
use rand::thread_rng;
use std::fs::read_to_string;
use tempfile::tempdir;

#[tokio::main]
async fn main() -> Result<()> {
    let dir = tempdir()?;
    let mut rng = thread_rng();

    // Password to encrypt the keystore file with.
    let password = "test";

    // Set up a local wallet from the first default Anvil account (Alice).
    let private_key =
        hex::decode("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")?;

    // Create a keystore file from the private key.
    let (key, uuid) =
        Wallet::encrypt_keystore(&dir, &mut rng, private_key, password, None).unwrap();

    let key_file_path = dir.path().join(uuid.clone());

    println!("Wrote keystore for {:?} to {:?}", key.address(), key_file_path);

    // Read the keystore file back.
    let wallet = Wallet::decrypt_keystore(key_file_path.clone(), password)?;

    println!("Read keystore from {:?}, recovered address: {:?}", key_file_path, wallet.address());

    // Assert that the address of the original key and the recovered key are the same.
    assert_eq!(wallet.address(), key.address());

    // Display the contents of the keystore file.
    let keystore_contents = read_to_string(key_file_path)?;

    print!("Keystore file contents: {:?}\n", keystore_contents);

    Ok(())
}
