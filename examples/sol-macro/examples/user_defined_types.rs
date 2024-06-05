//! Examples showing defining user defined value types (UDVTs) and type aliases using the sol macro
use alloy::{
    primitives::{Address, U256},
    sol,
    sol_types::SolType,
};

// Type definition: generates a new struct that implements `SolType`
sol! {
   /// Equivalent to `struct MyType(U256)` in Rust
   type MyType is uint256;
}

// Type aliases
type B32 = sol! { bytes32 };
// This is equivalent to the following:
// type B32 = alloy_sol_types::sol_data::FixedBytes<32>;

// User defined types
type SolArrayOf<T> = sol! { T[] };
type SolTuple = sol! { tuple(address, bytes, string) };

fn main() {
    let _b32 = B32::abi_encode(&[0; 32]);
    let _my_type = MyType(U256::from(1));
    let _arr = SolArrayOf::<sol!(bool)>::abi_encode(&vec![true, false]);
    let _tuple = SolTuple::abi_encode(&(Address::ZERO, vec![0; 32], "hello".to_string()));
}
