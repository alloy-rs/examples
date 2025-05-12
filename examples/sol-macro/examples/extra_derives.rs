//! This example shows how to apply trait derivations globally by specifying the path.
//!
//! While the `all_derives` attribute is useful for deriving standard Rust traits such as Debug,
//! `PartialEq`, Default etc., the `extra_derives` attribute allows us to derive other useful traits
//! by specifying their path.
//!
//! In this example, we'll derive `serde::Serialize` and `serde::Deserialize` for the types defined
//! in the `sol!` macro.

use alloy::sol;

sol!(
    // `all_derives` - derives standard Rust traits.
    #![sol(all_derives)]
    // `extra_derives` - derives additional traits by specifying their path.
    #![sol(extra_derives(serde::Serialize, serde::Deserialize))]
    Colors,
    "examples/abi/Colors.json",
);

fn main() -> eyre::Result<()> {
    let color_struct = Colors::Color { r: 255, ..Default::default() };

    // serde::Serialize is derived for types passed to the `sol!` macro.
    let json = serde_json::to_string_pretty(&color_struct)?;
    println!("{json}");

    // serde::Deserialize is derived for all types in the abi.
    let deserialized: Colors::Color = serde_json::from_str(&json)?;
    println!("{deserialized:?}");
    Ok(())
}
