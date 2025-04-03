//! This example shows how to apply trait derivations globally by specifying the path.

use alloy::sol;

sol!(
    #![sol(all_derives)]
    #![sol(extra_derives(serde::Serialize, serde::Deserialize))]
    Colors,
    "examples/abi/Colors.json",
);

fn main() -> eyre::Result<()> {
    let mut color_struct = Colors::Color::default();
    color_struct.r = 255;

    // serde::Serialize is derived for all types in the abi.
    let json = serde_json::to_string_pretty(&color_struct)?;
    println!("{json}");

    // serde::Deserialize is derived for all types in the abi.
    let deserialized: Colors::Color = serde_json::from_str(&json)?;
    println!("{:?}", deserialized);
    Ok(())
}
