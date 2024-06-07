//! Example showing how to use the `sol!` macro to generate Rust bindings for Solidity structs and
//! enums.

use alloy::{primitives::U256, sol};
use eyre::Result;

// Generates Rust bindings for Solidity structs, enums and type aliases.
sol! {
    #[allow(missing_docs)]
    #[derive(Debug)]
    /// Foo
    struct Foo {
        uint256 a;
        uint64 b;
        Bar greater;
    }

    #[allow(missing_docs)]
    #[derive(Debug)]
    /// Bar
    enum Bar {
        A,
        B,
    }
}

fn main() -> Result<()> {
    // Create an instance of the struct.
    let foo = Foo { a: U256::from(1), b: 2_u64, greater: Bar::A };

    println!("{:?}", foo);

    Ok(())
}
