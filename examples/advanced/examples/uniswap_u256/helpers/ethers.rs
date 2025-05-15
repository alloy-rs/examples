use alloy::primitives::U256 as AlloyU256;
use ethers::types::{Address, U256};
use std::ops::{Add, Div, Mul, Sub};

/// Uniswap V2 Pair
#[derive(Debug)]
pub struct UniV2Pair {
    /// Address of the pair contract
    pub address: Address,
    /// Token0 address
    pub token0: Address,
    /// Token1 address
    pub token1: Address,
    /// Reserves of token0
    pub reserve0: U256,
    /// Reserves of token1
    pub reserve1: U256,
}

// https://etherscan.io/address/0xA478c2975Ab1Ea89e8196811F51A7B7Ade33eB11
/// Get DAI-WETH Uniswap V2 pair
pub fn get_uniswap_pair() -> UniV2Pair {
    UniV2Pair {
        address: "0xA478c2975Ab1Ea89e8196811F51A7B7Ade33eB11".parse().unwrap(),
        token0: dai(),
        token1: weth(),
        reserve0: U256::from_dec_str("6227630995751221000110015").unwrap(),
        reserve1: U256::from_dec_str("2634810784674972449382").unwrap(),
    }
}

// https://etherscan.io/address/0xC3D03e4F041Fd4cD388c549Ee2A29a9E5075882f
/// Get DAI-WETH Sushiswap pair
pub fn get_sushi_pair() -> UniV2Pair {
    UniV2Pair {
        address: "0xC3D03e4F041Fd4cD388c549Ee2A29a9E5075882f".parse().unwrap(),
        token0: dai(),
        token1: weth(),
        reserve0: U256::from_dec_str("4314397529132715691120541").unwrap(),
        reserve1: U256::from_dec_str("1845242683965617816423").unwrap(),
    }
}

/// Helper trait to convert to alloy types
pub trait ToAlloy {
    /// Target type
    type To;
    /// Convert to target type
    fn to_alloy(self) -> Self::To;
}

impl ToAlloy for U256 {
    type To = AlloyU256;

    #[inline(always)]
    fn to_alloy(self) -> Self::To {
        AlloyU256::from_limbs(self.0)
    }
}

fn weth() -> Address {
    "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse().unwrap()
}

fn dai() -> Address {
    "0x6B175474E89094C44Da98b954EedeAC495271d0F".parse().unwrap()
}

/// Displays the token amount in a human-readable format
pub fn display_token(value: U256) -> String {
    format!("{:.16}", value.low_u128() as f64 / 1_000_000_000_000_000_000.0)
}

/// Gets the amountOut
pub fn get_amount_out(reserve_in: U256, reserve_out: U256, amount_in: U256) -> U256 {
    let amount_in_with_fee = amount_in * U256::from(997); // uniswap fee 0.3%
    let numerator = amount_in_with_fee * reserve_out;
    let denominator = reserve_in * U256::from(1000) + amount_in_with_fee;
    numerator / denominator
}

/// Gets the amountIn
pub fn get_amount_in(
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
    if input == U256::zero() {
        return U256::zero();
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
