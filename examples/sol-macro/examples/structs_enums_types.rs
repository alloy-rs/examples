//! Example showing how to use the sol macro to generate rust bindings for Solidity structs and
//! enums
use alloy::{
    primitives::{FixedBytes, U256},
    sol,
};

// Generates rust bindings for sol structs, enums, and type aliases.
sol! {
    #[allow(missing_docs)]
    #[derive(Debug)]
    /// Foo
    struct Foo {
        uint256 a;
        uint64 b;
        Bar greater;
    }

    #[derive(Debug)]
    /// Bar
    enum Bar {
        #[allow(missing_docs)]
        A,
        #[allow(missing_docs)]
        B,
    }

    #[derive(Debug)]
    type AliasSolType is bytes32;
}

fn main() {
    // Create an instance of the struct.
    let foo = Foo { a: U256::from(1), b: 2_u64, greater: Bar::A };

    let custom_sol_type: AliasSolType = FixedBytes::from([1; 32]).into();

    println!("{:?}", foo);
    println!("{:?}", custom_sol_type);
}

impl From<FixedBytes<32>> for AliasSolType {
    fn from(bytes: FixedBytes<32>) -> Self {
        Self { 0: bytes }
    }
}
