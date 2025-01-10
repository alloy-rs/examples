//! Example of mining a vanity Transaction hash using the data field of a `DynamicTyped`
//! input/function.

/// Dynamic types encode the length of the data at the start of the data:
/// appended bytes beyond the end of the datas length will be ignored by evm when decoding the
/// data. We can use this to mine a transaction hash with a specific prefix.
///    ➜ x = "`HelloWorld`"
///    Type: string
///    ├ UTF-8: `HelloWorld`
///    ├ Hex (Memory):
///    ├─ Length ([0x00:0x20]):
/// 0x000000000000000000000000000000000000000000000000000000000000000a    ├─ Contents
/// ([0x20:..]): 0x48656c6c6f576f726c6400000000000000000000000000000000000000000000
///
///    ➜ abi.encodeWithSignature("setName(string)", "`HelloWorld`")
///    Type: dynamic bytes
///    ├ Hex (Memory):
///    ├─ Length ([0x00:0x20]):
/// 0x0000000000000000000000000000000000000000000000000000000000000064    ├─ Contents
/// ([0x20:..]):
/// 0xc47f00270000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000a48656c6c6f576f726c640000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
///
///    cd dynamic-example && forge script scripts/Counter.s.sol --rpc-url `<http://127.0.0.1:8545>` --private-key $PK --broadcast
///    ...
///    cargo example --
///    Transaction: 0xdead2a52c2d7517e2b7d6a4092bb9061496906c018fd4c1d5cce5a2dec96da2c
///    Gas used: 46150
///
///    Block Number: 2
///    Block Hash: 0xa29bcce5697f3febdcd150ceb73f558b35d53dcadc485f0ba928f94f245c5de0
///    Block Time: "Mon, 16 Sep 2024 01:51:30 +0000"
use alloy::{
    consensus::TxEnvelope,
    hex,
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Bytes, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    signers::local::{coins_bip39::English, LocalSignerError, MnemonicBuilder},
    sol,
    sol_types::SolCall,
};
use eyre::Result;
use rand::Rng;

sol!(
    #[allow(missing_docs)]
    function setName(string calldata s) public;
);

async fn create_wallet() -> Result<PrivateKeySigner, LocalSignerError> {
    let wallet = MnemonicBuilder::<English>::default()
        .word_count(12)
        .derivation_path("m/44'/60'/0'/2/1")?
        .build_random()?;
    Ok(wallet)
}

#[tokio::main]
async fn main() -> Result<()> {
    // set to local anvil provider
    let rpc_url = "http://127.0.0.1:8545".parse()?;

    let new_wallet = create_wallet().await.unwrap();

    let wallet = EthereumWallet::from(new_wallet.clone());

    let provider =
        ProviderBuilder::new().with_recommended_fillers().wallet(wallet.clone()).on_http(rpc_url);

    let nonce = provider.get_transaction_count(new_wallet.address()).await?;
    let eip1559_est = provider.estimate_eip1559_fees(None).await?;

    let call = setNameCall { s: "hello".to_string() }.abi_encode();
    let input = Bytes::from(call);

    let mut tx_envelope;
    let mut rng = rand::thread_rng();

    loop {
        let mut input_vec = input.to_vec();
        let mut random_bytes = [0u8; 4];
        rng.fill(&mut random_bytes);

        input_vec.extend_from_slice(&random_bytes);
        let modified_input = Bytes::from(input_vec);

        let tx = TransactionRequest::default()
            // this should be the local address dynamicExample is deployed to via anvil
            .with_to("0xdEAD000000000000000042069420694206942069".parse()?)
            .with_nonce(nonce)
            .with_chain_id(31337)
            .with_value(U256::from(0))
            .with_gas_limit(100_000)
            .with_max_priority_fee_per_gas(eip1559_est.max_priority_fee_per_gas)
            .with_max_fee_per_gas(eip1559_est.max_fee_per_gas)
            .with_input(modified_input.clone());

        tx_envelope = tx.build(&wallet).await?;

        if let TxEnvelope::Eip1559(ref signed_tx) = tx_envelope {
            let tx_hash = hex::encode(signed_tx.hash());

            if tx_hash.starts_with("dead") {
                println!("Found a transaction hash starting with prefix: {:?}", tx_hash);
                let receipt = provider.send_tx_envelope(tx_envelope).await?.get_receipt().await?;
                println!("Sent transaction: {}", receipt.transaction_hash);
                break;
            }
        }
    }

    Ok(())
}
