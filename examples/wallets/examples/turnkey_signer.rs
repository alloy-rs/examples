//! Example showing how to use the Turnkey managed signer.

use alloy::{
    primitives::Address,
    signers::{turnkey::TurnkeySigner, Signer},
};
use example_support::required_env;
use eyre::{Result, WrapErr};

#[tokio::main]
async fn main() -> Result<()> {
    let organization_id = required_env("TURNKEY_ORGANIZATION_ID")?;
    let api_private_key = required_env("TURNKEY_API_PRIVATE_KEY")?;
    let address: Address = required_env("TURNKEY_ADDRESS")?
        .parse()
        .wrap_err("TURNKEY_ADDRESS must be a valid Ethereum address")?;

    let signer = TurnkeySigner::from_api_key(&api_private_key, organization_id, address, Some(1))?;

    let message = "Hello, world!";
    let signature = signer.sign_message(message.as_bytes()).await?;
    assert_eq!(signature.recover_address_from_msg(message)?, signer.address());

    Ok(())
}
