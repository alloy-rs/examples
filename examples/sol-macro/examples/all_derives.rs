//! This example demonstrates the `all_derives` attribute in the `sol!` macro.
//!
//! The `all_derives` attribute enables the derivation of standard Rust traits.
use alloy::{
    primitives::{Address, U256},
    sol,
};
use std::hash::{DefaultHasher, Hash, Hasher};

sol! (
    #![sol(all_derives)]
    // The `all_derives` attribute enables derivation of std rust traits such as
    // `Default`, `Debug`, `PartialEq`, `Eq`, and `Hash`.
    struct Foo {
        uint256 a;
        uint64 b;
        address c;
    }
);

fn main() {
    // `Default` is derived.
    let foo = Foo::default();
    let foo_bar = Foo { a: U256::from(1), b: 2, c: Address::with_last_byte(1) };

    let mut foo_list = vec![foo.clone(), foo, foo_bar];

    // `Debug` derived as well.
    println!("Initial foo_list: {foo_list:?}");

    // `PartialEq` is derived, enabling us to apply `.dedup()`.
    foo_list.dedup();
    assert_eq!(foo_list.len(), 2);

    // `Hash` is derived, enabling us to apply `.hash()`.
    let baz = Foo { a: U256::from(1), b: 2, c: Address::with_last_byte(1) };
    let mut hasher = DefaultHasher::default();
    baz.hash(&mut hasher);
    let hash = hasher.finish();
    println!("Hash of baz: {hash}");
}
