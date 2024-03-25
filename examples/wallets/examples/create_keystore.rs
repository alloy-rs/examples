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

    // Private key of Alice, the first default Anvil account.
    let private_key =
        hex::decode("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")?;

    // Password to encrypt the keystore file with.
    let password = "test";

    // Create a keystore file from the private key of Alice, returning a [Wallet] instance.
    let (wallet, file_path) =
        Wallet::encrypt_keystore(&dir, &mut rng, private_key, password, None).unwrap();

    let key_file_path = dir.path().join(file_path.clone());

    println!("Wrote keystore for {:?} to {:?}", wallet.address(), key_file_path);

    // Read the keystore file back.
    let recovered_wallet = Wallet::decrypt_keystore(key_file_path.clone(), password)?;

    println!(
        "Read keystore from {:?}, recovered address: {:?}",
        key_file_path,
        recovered_wallet.address()
    );

    // Assert that the address of the original key and the recovered key are the same.
    assert_eq!(wallet.address(), recovered_wallet.address());

    // Display the contents of the keystore file.
    let keystore_contents = read_to_string(key_file_path)?;

    print!("Keystore file contents: {:?}\n", keystore_contents);

    Ok(())
}
