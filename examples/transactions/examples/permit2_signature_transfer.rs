//! Example of how to transfer ERC20 tokens from one account to another using a signed permit.

use alloy::{
    network::EthereumWallet,
    node_bindings::Anvil,
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    signers::{
        local::{
            coins_bip39::{English, Mnemonic},
            PrivateKeySigner,
        },
        Signer,
    },
    sol,
    sol_types::eip712_domain,
};
use eyre::Result;
use std::str::FromStr;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20Example,
    "examples/artifacts/ERC20Example.json"
);

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Permit2,
    "examples/artifacts/Permit2.json"
);

// The permit stuct that has to be signed is different from the contract input struct
// even though they have the same name.
// Also note that the EIP712 hash of this struct is sensitive to the order of the fields.
sol! {
    struct TokenPermissions {
        address token;
        uint256 amount;
    }

    struct PermitTransferFrom {
        TokenPermissions permitted;
        address spender;
        uint256 nonce;
        uint256 deadline;
    }
}

impl From<PermitTransferFrom> for ISignatureTransfer::PermitTransferFrom {
    fn from(val: PermitTransferFrom) -> Self {
        Self {
            permitted: ISignatureTransfer::TokenPermissions {
                token: val.permitted.token,
                amount: val.permitted.amount,
            },
            nonce: val.nonce,
            deadline: val.deadline,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH.
    let rpc_url = "https://reth-ethereum.ithaca.xyz/rpc";
    // NOTE: ⚠️ Due to changes in EIP-7702 (see: https://getfoundry.sh/anvil/overview/#eip-7702-and-default-accounts),
    // the default mnemonic cannot be used for signature-based testing. Instead, we use a custom
    // mnemonic.
    let mnemonic = generate_mnemonic()?;
    let anvil = Anvil::new().fork(rpc_url).mnemonic(mnemonic).try_spawn()?;

    // Set up signers from the first two default Anvil accounts (Alice, Bob).
    let alice: PrivateKeySigner = anvil.keys()[8].clone().into();
    let bob: PrivateKeySigner = anvil.keys()[9].clone().into();

    // We can manage multiple signers with the same wallet
    let mut wallet = EthereumWallet::new(alice.clone());
    wallet.register_signer(bob.clone());

    // Create a provider with both signers pointing to anvil
    let rpc_url = anvil.endpoint_url();
    let provider = ProviderBuilder::new().wallet(wallet).connect_http(rpc_url);

    // Deploy the `ERC20Example` contract.
    let token = ERC20Example::deploy(provider.clone()).await?;

    // Register the balances of Alice and Bob before the transfer.
    let alice_before_balance = token.balanceOf(alice.address()).call().await?;
    let bob_before_balance = token.balanceOf(bob.address()).call().await?;

    // Permit2 mainnet address
    let address = Address::from_str("0x000000000022D473030F116dDEE9F6B43aC78BA3")?;
    let permit2 = Permit2::new(address, provider.clone());

    // Alice approves the Permit2 contract
    let tx_hash = token
        .approve(*permit2.address(), U256::MAX)
        .from(alice.address())
        .send()
        .await?
        .watch()
        .await?;
    println!("Sent approval: {tx_hash}");

    // Create the EIP712 Domain and Permit
    let amount = U256::from(100);
    let domain = eip712_domain! {
        name: "Permit2",
        chain_id: provider.get_chain_id().await?,
        verifying_contract: *permit2.address(),
    };
    let permit = PermitTransferFrom {
        permitted: TokenPermissions { token: *token.address(), amount },
        spender: bob.address(),
        nonce: U256::from(0),
        deadline: U256::MAX,
    };
    // Alice signs the Permit
    let signature = alice.sign_typed_data(&permit, &domain).await?.as_bytes().into();

    // This specifies the actual transaction executed via Permit2
    // Note that `to` can be any address and does not have to match the spender
    let transfer_details =
        ISignatureTransfer::SignatureTransferDetails { to: bob.address(), requestedAmount: amount };

    let tx_hash = permit2
        .permitTransferFrom_0(permit.into(), transfer_details, alice.address(), signature)
        .from(bob.address()) // the spender of the permit must be the msg.sender
        .send()
        .await?
        .watch()
        .await?;
    println!("Sent permit transfer: {tx_hash}");

    // Register the balances of Alice and Bob after the transfer.
    let alice_after_balance = token.balanceOf(alice.address()).call().await?;
    let bob_after_balance = token.balanceOf(bob.address()).call().await?;

    // Check the balances of Alice and Bob after the transfer.
    assert_eq!(alice_before_balance - alice_after_balance, amount);
    assert_eq!(bob_after_balance - bob_before_balance, amount);

    Ok(())
}

fn generate_mnemonic() -> Result<String> {
    let mut rng = rand::thread_rng();
    let mnemonic = Mnemonic::<English>::new_with_count(&mut rng, 12)?.to_phrase();
    Ok(mnemonic)
}
