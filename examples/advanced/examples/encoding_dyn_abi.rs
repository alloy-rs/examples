//! Example of [EIP712](https://eips.ethereum.org/EIPS/eip-712) encoding and decoding via `dyn_abi`.

use alloy::{
    dyn_abi::{DynSolType, DynSolValue},
    hex,
    primitives::{keccak256, Address, U256},
    signers::{local::PrivateKeySigner, Signer},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // EIP-712 domain
    let domain_type = DynSolType::Tuple(vec![
        DynSolType::String,    // name
        DynSolType::String,    // version
        DynSolType::Uint(256), // chainId
        DynSolType::Address,   // verifyingContract
    ]);

    let domain_value = DynSolValue::Tuple(vec![
        DynSolValue::String("Alloy".to_string()),
        DynSolValue::String("1.0.1".to_string()),
        DynSolValue::Uint(U256::from(1), 256),
        DynSolValue::Address(Address::from([0x42; 20])),
    ]);

    // Message type (sample message)
    let message_type = DynSolType::Tuple(vec![
        DynSolType::Address, // from
        DynSolType::Address, // to
        DynSolType::String,  // contents
    ]);

    // Random values
    let message_value = DynSolValue::Tuple(vec![
        DynSolValue::Address(Address::from([0x11; 20])),
        DynSolValue::Address(Address::from([0x22; 20])),
        DynSolValue::String("EIP-712 encoding".to_string()),
    ]);

    // Encode the domain and message
    let encoded_domain = domain_value.abi_encode();
    let encoded_message = message_value.abi_encode();

    println!("Encoded domain: 0x{}", hex::encode(&encoded_domain));
    println!("Encoded message: 0x{}", hex::encode(&encoded_message));

    // Decode the domain and message
    let decoded_domain = domain_type.abi_decode(&encoded_domain)?;
    let decoded_message = message_type.abi_decode(&encoded_message)?;

    println!("\nDecoded domain:");
    print_tuple(&decoded_domain, &["name", "version", "chainId", "verifyingContract"]);

    println!("\nDecoded message:");
    print_tuple(&decoded_message, &["from", "to", "contents"]);

    // Calculate EIP-712 hash
    let domain_separator = keccak256(&encoded_domain);
    let message_hash = keccak256(&encoded_message);
    let eip712_hash = keccak256([&[0x19, 0x01], &domain_separator[..], &message_hash[..]].concat());

    println!("\nEIP-712 hash: 0x{}", hex::encode(eip712_hash));

    // Signing the hash via random signer
    // Ref: examples/wallets/examples/sign_message.rs

    // Create a signer
    let wallet = PrivateKeySigner::random();
    println!("\nSigner address: {}", wallet.address());

    // Sign the EIP-712 hash
    let signature = wallet.sign_hash(&eip712_hash).await?;
    println!("Signature: 0x{}", hex::encode(signature.as_bytes()));

    // Verify the signature
    let recovered_address = signature.recover_address_from_prehash(&eip712_hash)?;
    println!("Recovered address: {recovered_address}");

    assert_eq!(recovered_address, wallet.address(), "Signature verification failed");
    println!("Signature verified successfully!");

    Ok(())
}

/// Utility function to print the decoded data.
fn print_tuple(value: &DynSolValue, field_names: &[&str]) {
    if let DynSolValue::Tuple(values) = value {
        for (value, name) in values.iter().zip(field_names.iter()) {
            println!("  {name}: {value:?}");
        }
    }
}
