//! Example of using `AnyNetwork` to get a type-safe representation of
//! network-specific data.
//!
//! In this example, we extract the `gasUsedForL1` and `l1BlockNumber` fields
//! of Arbitrum's transaction receipts.

use alloy::{
    network::AnyNetwork,
    primitives::{address, Address, U128, U256, U64},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};
use eyre::Result;

// The address of the contract below deployed to Arbitrum Sepolia.
const COUNTER_CONTRACT_ADDRESS: Address = address!("d62FC4aB418580919F22E2aC3A0D93F832A95E70");

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract Counter {
        uint256 public number;

        function setNumber(uint256 newNumber) public {
            number = newNumber;
        }

        function increment() public {
            number++;
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct ArbOtherFields {
    #[serde(rename = "gasUsedForL1")]
    gas_used_for_l1: U128,
    #[serde(rename = "l1BlockNumber")]
    l1_block_number: U64,
}

#[tokio::main]
async fn main() -> Result<()> {
    // [RISK WARNING! Writing a private key in the code file is insecure behavior.]
    // The following code is for testing only. Set up signer from private key, be aware of danger.
    let signer: PrivateKeySigner = "<PRIVATE_KEY>".parse().expect("should parse private key");

    // Create a provider with the Arbitrum Sepolia network and the wallet.
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc".parse()?;
    let provider =
        ProviderBuilder::new().network::<AnyNetwork>().wallet(signer).connect_http(rpc_url);

    // Create a contract instance.
    let contract = Counter::new(COUNTER_CONTRACT_ADDRESS, &provider);

    // Set the number to 42.
    let builder = contract.setNumber(U256::from(42));
    let receipt = builder.send().await?.get_receipt().await?;

    // Fetch the `gasUsedForL1` and `l1BlockNumber` fields from the receipt.
    let arb_fields: ArbOtherFields = receipt.other.deserialize_into()?;
    let l1_gas = arb_fields.gas_used_for_l1.to::<u128>();
    let l1_block_number = arb_fields.l1_block_number.to::<u64>();

    println!("Gas used for L1: {l1_gas}");
    println!("L1 block number: {l1_block_number}");

    Ok(())
}
