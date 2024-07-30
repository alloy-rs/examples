//! Example showing defining user defined value types and type aliases using the `sol!` macro.

use alloy::{
    primitives::{Address, U256},
    sol,
    sol_types::SolType,
};
use eyre::Result;

// Type definition: generates a new struct that implements `SolType`
sol! {
   /// Equivalent to `struct CustomType(U256)` in Rust
   type CustomType is uint256;
}

// Type aliases
type Bytes32 = sol! { bytes32 };

// This is equivalent to the following:
// type B32 = alloy_sol_types::sol_data::FixedBytes<32>;

// User defined types
type CustomArrayOf<T> = sol! { T[] };
type CustomTuple = sol! { tuple(address, bytes, string) };

fn main() -> Result<()> {
    let _b32_type = Bytes32::abi_encode(&[0; 32]);

    let _custom_type = CustomType(U256::from(1));

    let _custom_array_of_type = CustomArrayOf::<sol!(bool)>::abi_encode(&vec![true, false]);

    let _custom_tuple_type =
        CustomTuple::abi_encode(&(Address::ZERO, vec![0; 32], "hello".to_string()));

    Ok(())
}
