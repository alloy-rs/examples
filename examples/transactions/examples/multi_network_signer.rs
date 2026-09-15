//! Example of using a single signer across multiple EVM-compatible networks.
//!
//! This example demonstrates how to:
//! - Create a single signer that can be reused across networks
//! - Connect to multiple EVM-compatible networks (Ethereum, Optimism, Arbitrum)
//! - Send transactions to each network using the same wallet
//! - Handle network-specific transaction receipt fields
//! - Compare gas costs across different networks
//!
//! # Running the example
//!
//! First, set your private key as an environment variable:
//! ```bash
//! export PRIVATE_KEY="0x..."
//! ```
//!
//! Then run:
//! ```bash
//! cargo run --example multi_network_signer
//! ```
//!
//! # Safety Warning
//!
//! NEVER commit your private key to version control or share it publicly!
//! This example uses an environment variable to keep your key secure.
//!
//! # Getting Testnet ETH
//!
//! You'll need testnet ETH on each network:
//! - Ethereum Sepolia: https://sepoliafaucet.com/
//! - Optimism Sepolia: https://app.optimism.io/faucet
//! - Arbitrum Sepolia: https://faucet.quicknode.com/arbitrum/sepolia

use alloy::{
    network::AnyNetwork,
    primitives::{address, Address, U256, U128, U64},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use alloy::network::TransactionBuilder;
use eyre::Result;

// Network configuration with metadata
struct NetworkConfig {
    name: &'static str,
    rpc_url: &'static str,
    chain_id: u64,
    is_l2: bool,
}

const NETWORKS: &[NetworkConfig] = &[
    NetworkConfig {
        name: "Ethereum Sepolia",
        rpc_url: "https://ethereum-sepolia-rpc.publicnode.com",
        chain_id: 11155111,
        is_l2: false,
    },
    NetworkConfig {
        name: "Optimism Sepolia",
        rpc_url: "https://sepolia.optimism.io",
        chain_id: 11155420,
        is_l2: true,
    },
    NetworkConfig {
        name: "Arbitrum Sepolia",
        rpc_url: "https://sepolia-rollup.arbitrum.io/rpc",
        chain_id: 421614,
        is_l2: true,
    },
];

// Minimum balance required to send a transaction (0.001 ETH)
const MIN_BALANCE: u128 = 1_000_000_000_000_000; // 0.001 ETH in wei

// Amount to send in each transaction (0.0001 ETH)
const SEND_AMOUNT: u128 = 100_000_000_000_000; // 0.0001 ETH in wei

// Recipient address for test transactions (you can use your own address or a burner)
const RECIPIENT_ADDRESS: Address = address!("0x0000000000000000000000000000000000000001");

// Struct to deserialize Arbitrum-specific receipt fields
#[derive(Debug, serde::Deserialize)]
struct ArbReceiptFields {
    #[serde(rename = "gasUsedForL1")]
    gas_used_for_l1: U128,
    #[serde(rename = "l1BlockNumber")]
    l1_block_number: U64,
}

// Struct to deserialize Optimism-specific receipt fields
#[derive(Debug, serde::Deserialize)]
struct OpReceiptFields {
    #[serde(rename = "l1GasUsed")]
    l1_gas_used: Option<U256>,
    #[serde(rename = "l1GasPrice")]
    l1_gas_price: Option<U256>,
    #[serde(rename = "l1Fee")]
    l1_fee: Option<U256>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging for better debugging
    env_logger::init();

    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║   Multi-Network Transaction Example with Single Signer   ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();

    // Create a single signer from private key
    let private_key = std::env::var("PRIVATE_KEY")
        .expect("PRIVATE_KEY environment variable not set. Set it with: export PRIVATE_KEY=\"0x...\"");
    
    let signer: PrivateKeySigner = private_key
        .parse()
        .expect("Failed to parse private key. Make sure it's in the format: 0x...");
    
    let wallet_address = signer.address();
    println!("Using wallet address: {wallet_address}");
    println!("Recipient address: {RECIPIENT_ADDRESS}");
    println!();

    // Iterate through each network
    for network in NETWORKS {
        println!("╔═══════════════════════════════════════════════════════════╗");
        println!("║ {:<57} ║", network.name);
        println!("╚═══════════════════════════════════════════════════════════╝");
        
        // Create provider with AnyNetwork and the shared signer
        let provider = ProviderBuilder::new()
            .network::<AnyNetwork>()
            .wallet(signer.clone())
            .connect_http(network.rpc_url.parse::<Url>()?);
        
        // Get initial balance
        let initial_balance = match provider.get_balance(wallet_address).await {
            Ok(balance) => {
                println!("Initial balance: {} wei ({:.6} ETH)", 
                    balance, 
                    balance.to::<u128>() as f64 / 1e18
                );
                balance
            }
            Err(e) => {
                println!("Error getting balance: {}", e);
                println!();
                continue;
            }
        };

        // Check if we have sufficient balance to send a transaction
        if initial_balance.to::<u128>() < MIN_BALANCE {
            println!("Insufficient balance to send transaction");
            println!("Need at least {} wei ({:.6} ETH)", MIN_BALANCE, MIN_BALANCE as f64 / 1e18);
            println!("Visit the faucets listed at the top of this file");
            println!();
            continue;
        }

        println!("Sufficient balance to send transaction");
        println!();

        // Build and send transaction
        println!("Building transaction...");
        let tx = TransactionRequest::default()
            .with_to(RECIPIENT_ADDRESS)
            .with_value(U256::from(SEND_AMOUNT))
            .with_chain_id(network.chain_id);

        // Convert the plain TransactionRequest into the network-specific request type
        // (e.g. WithOtherFields<TransactionRequest>) that the provider expects.
        let tx = tx.into();

        println!("   Amount: {} wei ({:.6} ETH)", SEND_AMOUNT, SEND_AMOUNT as f64 / 1e18);
        println!();

        // Send the transaction
        println!("Sending transaction...");
        let pending_tx = match provider.send_transaction(tx).await {
            Ok(pending) => {
                println!(" Transaction hash: {:?}", pending.tx_hash());
                pending
            }
            Err(e) => {
                println!("Error sending transaction: {}", e);
                println!();
                continue;
            }
        };

        // Wait for transaction confirmation
        println!("Waiting for confirmation...");
        let receipt = match pending_tx.get_receipt().await {
            Ok(receipt) => {
                println!("Transaction confirmed!");
                receipt
            }
            Err(e) => {
                println!("Error getting receipt: {}", e);
                println!();
                continue;
            }
        };

        // Display receipt information
        println!();
        println!("Transaction Receipt:");
        println!("Block number: {}", receipt.block_number.unwrap_or_default());
        println!("Gas used: {}", receipt.gas_used);
        println!("Effective gas price: {} wei", receipt.effective_gas_price);
        
        // Calculate total gas cost
        let gas_cost = (receipt.gas_used as u128) * receipt.effective_gas_price;
        println!("   Total gas cost: {} wei ({:.6} ETH)", 
            gas_cost, 
            gas_cost as f64 / 1e18
        );

        // Extract and display network-specific fields
        if network.is_l2 {
            println!();
            println!("🔍 L2-Specific Fields:");
            
            if network.name.contains("Arbitrum") {
                // Extract Arbitrum-specific fields
                match receipt.other.deserialize_into::<ArbReceiptFields>() {
                    Ok(arb_fields) => {
                        let l1_gas = arb_fields.gas_used_for_l1.to::<u128>();
                        let l1_block = arb_fields.l1_block_number.to::<u64>();
                        println!("   L1 Gas Used: {} wei", l1_gas);
                        println!("   L1 Block Number: {}", l1_block);
                        
                        // Calculate approximate L1 cost
                        println!("Arbitrum uses L1 for data availability");
                    }
                    Err(e) => {
                        println!(" Could not parse Arbitrum-specific fields: {}", e);
                    }
                }
            } else if network.name.contains("Optimism") {
                // Extract Optimism-specific fields
                match receipt.other.deserialize_into::<OpReceiptFields>() {
                    Ok(op_fields) => {
                        if let Some(l1_fee) = op_fields.l1_fee {
                            println!("   L1 Fee: {} wei ({:.6} ETH)", 
                                l1_fee, 
                                l1_fee.to::<u128>() as f64 / 1e18
                            );
                        }
                        if let Some(l1_gas_used) = op_fields.l1_gas_used {
                            println!(" L1 Gas Used: {} units", l1_gas_used);
                        }
                        if let Some(l1_gas_price) = op_fields.l1_gas_price {
                            println!(" L1 Gas Price: {} wei", l1_gas_price);
                        }
                        println!("  Optimism uses L1 for data availability");
                    }
                    Err(e) => {
                        println!("  Could not parse Optimism-specific fields: {}", e);
                    }
                }
            }
        }

        // Get final balance
        println!();
        let final_balance = match provider.get_balance(wallet_address).await {
            Ok(balance) => {
                println!("Final balance: {} wei ({:.6} ETH)", 
                    balance, 
                    balance.to::<u128>() as f64 / 1e18
                );
                balance
            }
            Err(e) => {
                println!("Error getting final balance: {}", e);
                initial_balance // Use initial balance if we can't fetch final
            }
        };

        // Calculate total cost
        let total_cost = initial_balance.saturating_sub(final_balance);
        println!("💸 Total cost: {} wei ({:.6} ETH)", 
            total_cost, 
            total_cost.to::<u128>() as f64 / 1e18
        );
        println!("   (Includes {} wei sent + gas fees)", SEND_AMOUNT);
        
        println!();
        println!("─────────────────────────────────────────────────────────────");
        println!();

        // Small delay between networks to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }

    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║                    Summary Complete                       ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
    println!("Successfully demonstrated single signer across {} networks", NETWORKS.len());
    println!("Key Observations:");
    println!("   • Same private key worked on all EVM-compatible networks");
    println!("   • L2 transactions typically have lower gas costs");
    println!("   • L2s have additional fields for L1 data availability costs");
    println!();

    Ok(())
}