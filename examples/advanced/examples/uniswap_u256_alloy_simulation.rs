//! Uniswap V2 Arbitrage Simulation using alloy

use alloy::{
    hex,
    network::TransactionBuilder,
    primitives::{address, utils::parse_units, Bytes, B256, U256},
    providers::{ext::AnvilApi, Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
};
use eyre::Result;
use helpers::alloy::{
    get_amount_in, get_amount_out, get_sushi_pair, get_uniswap_pair, set_hash_storage_slot,
    DAI_ADDR, WETH_ADDR,
};

sol! {
    function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external;
}

sol!(
    #[sol(rpc)]
    contract IERC20 {
        function balanceOf(address target) returns (uint256);
    }
);

sol!(
    #[sol(rpc)]
    FlashBotsMultiCall,
    "examples/abi/FlashBotsMultiCall.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    let uniswap_pair = get_uniswap_pair();
    let sushi_pair = get_sushi_pair();

    let wallet_address = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    let provider = ProviderBuilder::new()
        .connect_anvil_with_wallet_and_config(|a| a.fork("https://reth-ethereum.ithaca.xyz/rpc"))?;

    let executor = FlashBotsMultiCall::deploy(provider.clone(), wallet_address).await?;
    let iweth = IERC20::new(WETH_ADDR, provider.clone());

    // Mock WETH balance for executor contract
    set_hash_storage_slot(
        provider.clone(),
        WETH_ADDR,
        U256::from(3),
        *executor.address(),
        parse_units("5.0", "ether")?.into(),
    )
    .await?;

    // Mock reserves for Uniswap pair
    provider
        .anvil_set_storage_at(
            uniswap_pair.address,
            U256::from(8), // getReserves slot
            B256::from_slice(&hex!(
                "665c6fcf00000000008ed55850d607f83a660000000526c08d812099d2577fbf"
            )),
        )
        .await?;

    // Mock WETH balance for Uniswap pair
    set_hash_storage_slot(
        &provider,
        WETH_ADDR,
        U256::from(3),
        uniswap_pair.address,
        uniswap_pair.reserve1,
    )
    .await?;

    // Mock DAI balance for Uniswap pair
    set_hash_storage_slot(
        &provider,
        DAI_ADDR,
        U256::from(2),
        uniswap_pair.address,
        uniswap_pair.reserve0,
    )
    .await?;

    // Mock reserves for Sushi pair

    provider
        .anvil_set_storage_at(
            sushi_pair.address,
            U256::from(8), // getReserves slot
            B256::from_slice(&hex!(
                "665c6fcf00000000006407e2ec8d4f09436700000003919bf56d886af022979d"
            )),
        )
        .await?;

    // Mock WETH balance for Sushi pair
    set_hash_storage_slot(
        &provider,
        WETH_ADDR,
        U256::from(3),
        sushi_pair.address,
        sushi_pair.reserve1,
    )
    .await?;

    // Mock DAI balance for Sushi pair
    set_hash_storage_slot(
        &provider,
        DAI_ADDR,
        U256::from(2),
        sushi_pair.address,
        sushi_pair.reserve0,
    )
    .await?;

    let balance_of = iweth.balanceOf(*executor.address()).call().await?;
    println!("Before - WETH balance of executor {:?}", balance_of);

    let weth_amount_in = get_amount_in(
        uniswap_pair.reserve0,
        uniswap_pair.reserve1,
        false,
        sushi_pair.reserve0,
        sushi_pair.reserve1,
    );

    let dai_amount_out =
        get_amount_out(uniswap_pair.reserve1, uniswap_pair.reserve0, weth_amount_in);

    let weth_amount_out = get_amount_out(sushi_pair.reserve0, sushi_pair.reserve1, dai_amount_out);

    let swap1 = swapCall {
        amount0Out: dai_amount_out,
        amount1Out: U256::ZERO,
        to: sushi_pair.address,
        data: Bytes::new(),
    }
    .abi_encode();

    let swap2 = swapCall {
        amount0Out: U256::ZERO,
        amount1Out: weth_amount_out,
        to: *executor.address(),
        data: Bytes::new(),
    }
    .abi_encode();

    let arb_calldata = FlashBotsMultiCall::uniswapWethCall {
        _wethAmountToFirstMarket: weth_amount_in,
        _ethAmountToCoinbase: U256::ZERO,
        _targets: vec![uniswap_pair.address, sushi_pair.address],
        _payloads: vec![Bytes::from(swap1), Bytes::from(swap2)],
    }
    .abi_encode();

    let arb_tx =
        TransactionRequest::default().with_to(*executor.address()).with_input(arb_calldata);

    let pending = provider.send_transaction(arb_tx).await?;
    pending.get_receipt().await?;

    let balance_of = iweth.balanceOf(*executor.address()).call().await?;
    println!("After - WETH balance of executor {:?}", balance_of);

    Ok(())
}
