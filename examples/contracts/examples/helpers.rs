//! Helpers for the `UniswapV2` and Sushiswap arb simulation.
#![allow(missing_docs, dead_code)]
use std::ops::{Add, Div, Mul, Sub};

use alloy::{
    primitives::{address, keccak256, Address, U256},
    providers::{ext::AnvilApi, Provider},
    sol_types::SolValue,
    uint,
};
use eyre::Result;

pub(crate) static WETH_ADDR: Address = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
pub(crate) static DAI_ADDR: Address = address!("6B175474E89094C44Da98b954EedeAC495271d0F");

#[derive(Debug)]
pub(crate) struct UniV2Pair {
    pub(crate) address: Address,
    pub(crate) token0: Address,
    pub(crate) token1: Address,
    pub(crate) reserve0: U256,
    pub(crate) reserve1: U256,
}

// https://etherscan.io/address/0xA478c2975Ab1Ea89e8196811F51A7B7Ade33eB11
pub(crate) fn get_uniswap_pair() -> UniV2Pair {
    UniV2Pair {
        address: address!("A478c2975Ab1Ea89e8196811F51A7B7Ade33eB11"),
        token0: DAI_ADDR,
        token1: WETH_ADDR,
        reserve0: uint!(6227630995751221000110015_U256),
        reserve1: uint!(2634810784674972449382_U256),
    }
}

// https://etherscan.io/address/0xC3D03e4F041Fd4cD388c549Ee2A29a9E5075882f
pub(crate) fn get_sushi_pair() -> UniV2Pair {
    UniV2Pair {
        address: address!("C3D03e4F041Fd4cD388c549Ee2A29a9E5075882f"),
        token0: DAI_ADDR,
        token1: WETH_ADDR,
        reserve0: uint!(4314397529132715691120541_U256),
        reserve1: uint!(1845242683965617816423_U256),
    }
}

pub(crate) fn get_amount_out(reserve_in: U256, reserve_out: U256, amount_in: U256) -> U256 {
    let amount_in_with_fee = amount_in * get_uniswappy_fee();
    let numerator = amount_in_with_fee * reserve_out;
    let denominator = reserve_in * U256::from(1000) + amount_in_with_fee;
    numerator / denominator
}

pub(crate) fn get_amount_in(
    reserves00: U256,
    reserves01: U256,
    is_weth0: bool,
    reserves10: U256,
    reserves11: U256,
) -> U256 {
    let numerator = get_numerator(reserves00, reserves01, is_weth0, reserves10, reserves11);

    let denominator = get_denominator(reserves00, reserves01, is_weth0, reserves10, reserves11);

    numerator * U256::from(1000) / denominator
}

fn sqrt(input: U256) -> U256 {
    if input == U256::ZERO {
        return U256::ZERO;
    }

    let mut z = (input + U256::from(1)) / U256::from(2);
    let mut y = input;
    while z < y {
        y = z;
        z = (input / z + z) / U256::from(2);
    }
    y
}

fn get_numerator(
    reserves00: U256,
    reserves01: U256,
    is_weth0: bool,
    reserves10: U256,
    reserves11: U256,
) -> U256 {
    if is_weth0 {
        let presqrt = get_uniswappy_fee()
            .mul(get_uniswappy_fee())
            .mul(reserves01)
            .mul(reserves10)
            .div(reserves11)
            .div(reserves00);
        sqrt(presqrt).sub(U256::from(1000)).mul(reserves11).mul(reserves00)
    } else {
        let presqrt = get_uniswappy_fee()
            .mul(get_uniswappy_fee())
            .mul(reserves00)
            .mul(reserves11)
            .div(reserves10)
            .div(reserves01);
        (sqrt(presqrt)).sub(U256::from(1000)).mul(reserves10).mul(reserves01)
    }
}

fn get_denominator(
    reserves00: U256,
    reserves01: U256,
    is_weth0: bool,
    reserves10: U256,
    reserves11: U256,
) -> U256 {
    if is_weth0 {
        get_uniswappy_fee()
            .mul(reserves11)
            .mul(U256::from(1000))
            .add(get_uniswappy_fee().mul(get_uniswappy_fee()).mul(reserves01))
    } else {
        get_uniswappy_fee()
            .mul(reserves10)
            .mul(U256::from(1000))
            .add(get_uniswappy_fee().mul(get_uniswappy_fee()).mul(reserves00))
    }
}

fn get_uniswappy_fee() -> U256 {
    U256::from(997)
}

pub(crate) async fn set_hash_storage_slot<P: Provider>(
    anvil_provider: &P,
    address: Address,
    hash_slot: U256,
    hash_key: Address,
    value: U256,
) -> Result<()> {
    let hashed_slot = keccak256((hash_key, hash_slot).abi_encode());

    anvil_provider.anvil_set_storage_at(address, hashed_slot.into(), value.into()).await?;

    Ok(())
}

const fn main() {}
