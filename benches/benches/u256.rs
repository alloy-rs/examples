use alloy::primitives::U256 as aU256;

use criterion::{criterion_group, criterion_main, Criterion};
use ethers::types::U256;
use std::{hint::black_box, str::FromStr};

use alloy_benches::{
    alloy_helpers::{
        get_amount_in as a_get_amount_in, get_amount_out as a_get_amount_out,
        get_sushi_pair as a_get_sushi_pair, get_uniswap_pair as a_get_uniswap_pair,
    },
    ethers_helpers::{
        get_amount_in as e_get_amount_in, get_amount_out as e_get_amount_out,
        get_sushi_pair as e_get_sushi_pair, get_uniswap_pair as e_get_uniswap_pair,
    },
};

fn u256_benchmark(c: &mut Criterion) {
    let a_uniswap_pair = a_get_uniswap_pair();
    let a_sushi_pair = a_get_sushi_pair();
    let e_uniswap_pair = e_get_uniswap_pair();
    let e_sushi_pair = e_get_sushi_pair();

    let a_amount_in = aU256::from_str("1000000000000000000").unwrap();
    let e_amount_in = U256::from_dec_str("1000000000000000000").unwrap();

    let mut group1 = c.benchmark_group("U256 Operations");

    group1.bench_function("Ethers/getAmountIn", |b| {
        b.iter(|| {
            _ = e_get_amount_in(
                black_box(e_uniswap_pair.reserve0),
                black_box(e_uniswap_pair.reserve1),
                black_box(false),
                black_box(e_sushi_pair.reserve0),
                black_box(e_sushi_pair.reserve1),
            );
        })
    });

    group1.bench_function("Alloy/getAmountIn", |b| {
        b.iter(|| {
            _ = a_get_amount_in(
                black_box(a_uniswap_pair.reserve0),
                black_box(a_uniswap_pair.reserve1),
                black_box(false),
                black_box(a_sushi_pair.reserve0),
                black_box(a_sushi_pair.reserve1),
            );
        })
    });

    group1.bench_function("Ethers/getAmountOut", |b| {
        b.iter(|| {
            _ = e_get_amount_out(
                black_box(e_uniswap_pair.reserve0),
                black_box(e_uniswap_pair.reserve1),
                black_box(e_amount_in),
            );
        })
    });

    group1.bench_function("Alloy/getAmountOut", |b| {
        b.iter(|| {
            _ = a_get_amount_out(
                black_box(a_uniswap_pair.reserve0),
                black_box(a_uniswap_pair.reserve1),
                black_box(a_amount_in),
            );
        })
    });

    group1.finish();
}

criterion_group!(benches, u256_benchmark);
criterion_main!(benches);
