//! Demonstrates how `EthereumWallet` can use multiple different types of signers.

use alloy::{
    network::{EthereumWallet, TransactionBuilder, TxSigner},
    node_bindings::Anvil,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::{
        aws::AwsSigner,
        ledger::{HDPath, LedgerSigner},
        local::PrivateKeySigner,
    },
};
use aws_config::BehaviorVersion;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let anvil = Anvil::new().try_spawn()?;

    // Initialize your signers.

    // Set up a local signer.
    let pk_signer: PrivateKeySigner =
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;
    let pk_addr = pk_signer.address();

    // Set up a hardware wallet signer
    let ledger = LedgerSigner::new(HDPath::LedgerLive(0), Some(1)).await?;
    let ledger_addr = ledger.address();

    // AWS KMS signer
    let key_id = std::env::var("AWS_KEY_ID").expect("AWS_KEY_ID not set in .env file");
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_kms::Client::new(&config);
    let aws = AwsSigner::new(client, key_id, Some(1)).await?;
    let _aws_addr = aws.address();

    // Initialize `EthereumWallet`.
    // `pk_signer` is set as the default signer.
    // This signer is used to sign `TransactionRequest` and `TypedTransaction` objects that do
    // not specify a signer address in the `from` field.
    let mut wallet = EthereumWallet::new(pk_signer);
    // Add aws and ledger signers to the wallet
    wallet.register_signer(aws);
    wallet.register_signer(ledger);

    // Create a provider with the `WalletFiller`.
    let provider = ProviderBuilder::new().wallet(wallet).connect_http(anvil.endpoint_url());

    // Note that the `from` field hasn't been specified.
    // The wallet filler in the provider will set to it the default signer's address, which is
    // `pk_signer`.
    let tx = TransactionRequest::default()
        .with_to(address!("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"))
        .with_value(U256::from(100));

    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;
    assert_eq!(pk_addr, receipt.from);

    // One can hint the wallet filler about which signer to use by specifying the `from` field.
    // In this case, the `ledger` signer will be used to sign the transaction.
    let tx = TransactionRequest::default()
        .with_from(ledger_addr)
        .with_to(address!("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"))
        .with_value(U256::from(100));

    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;
    assert_eq!(ledger_addr, receipt.from);

    Ok(())
}
