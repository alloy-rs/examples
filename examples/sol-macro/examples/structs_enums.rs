//! Example showing how to use the sol macro to generate rust bindings for Solidity structs and
//! enums
use alloy::{primitives::U256, sol};

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
}

fn main() {
    // Create an instance of the struct.
    #[allow(clippy::disallowed_names)]
    let foo = Foo { a: U256::from(1), b: 2_u64, greater: Bar::A };

    println!("{:?}", foo);
}
