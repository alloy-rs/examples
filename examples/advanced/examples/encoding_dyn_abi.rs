//! Example of using `DynSolType` to encode and decode calldata

use alloy::{
    dyn_abi::{DynSolType, DynSolValue},
    hex,
    primitives::{keccak256, Address, U256},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Function signature
    let function_signature = "sampleMethod(address,uint256[],bytes,bool,(address,uint256),string)";

    // Calculate function selector (first 4 bytes of the Keccak256 hash of the function signature)
    let selector = &keccak256(function_signature.as_bytes())[0..4];

    // Example input params
    let recipient = Address::from([0x42; 20]);
    let amounts = vec![U256::from(1e18 as u64), U256::from(2e18 as u64)];
    let data = vec![0x13, 0x37];
    let active = true;
    let addr = Address::from([0x24; 20]);
    let nonce = U256::from(42);
    let some_metadata = "Alloy Gud".to_string();

    let param_types = DynSolType::Tuple(vec![
        DynSolType::Address,
        DynSolType::Array(Box::new(DynSolType::Uint(256))),
        DynSolType::Bytes,
        DynSolType::Bool,
        DynSolType::Tuple(vec![DynSolType::Address, DynSolType::Uint(256)]),
        DynSolType::String,
    ]);

    let param_values = DynSolValue::Tuple(vec![
        DynSolValue::Address(recipient),
        DynSolValue::Array(amounts.into_iter().map(|a| DynSolValue::Uint(a, 256)).collect()),
        DynSolValue::Bytes(data),
        DynSolValue::Bool(active),
        DynSolValue::Tuple(vec![DynSolValue::Address(addr), DynSolValue::Uint(nonce, 256)]),
        DynSolValue::String(some_metadata),
    ]);

    // Encode parameters via `abi_encode()`
    let encoded_params = param_values.abi_encode();

    // Prepend function selector with encoded parameters to get full calldata
    let mut full_calldata = selector.to_vec();
    full_calldata.extend_from_slice(&encoded_params);

    println!("Function Signature: {}", function_signature);
    println!("Function Selector: 0x{}", hex::encode(selector));
    println!("Calldata: 0x{}", hex::encode(&full_calldata));

    // Decode parameters (excluding the function selector)
    let decoded = param_types.abi_decode(&encoded_params)?;
    println!("\nDecoded Parameters:");
    if let DynSolValue::Tuple(values) = decoded {
        for (i, value) in values.iter().enumerate() {
            println!("Param {}: {:?}", i, value);
        }
    }

    Ok(())
}
