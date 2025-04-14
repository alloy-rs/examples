use alloy::{
    primitives::{Address, Bytes, U256},
    sol,
    sol_types::SolCall,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ethers::{
    contract::EthCall,
    core::abi::AbiEncode,
    types::{Bytes as eBytes, H160 as eH160, U256 as eU256},
};

sol! {
    #[sol(rpc)]
    function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external;
}

#[derive(EthCall)]
#[ethcall(name = "swap")]
struct SwapCall {
    amount0_out: eU256,
    amount1_out: eU256,
    to: eH160,
    data: eBytes,
}

fn encoding_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Static");

    group.bench_function("Ethers", |b| {
        b.iter(|| {
            SwapCall {
                amount0_out: black_box(eU256::from(1)),
                amount1_out: black_box(eU256::from(0)),
                to: black_box(eH160::from([0x42; 20])),
                data: black_box(eBytes::new()),
            }
            .encode();
        })
    });

    group.bench_function("Alloy", |b| {
        b.iter(|| {
            swapCall {
                amount0Out: black_box(U256::from(1)),
                amount1Out: black_box(U256::from(0)),
                to: black_box(Address::from([0x42; 20])),
                data: black_box(Bytes::new()),
            }
            .abi_encode();
        })
    });

    group.finish();
}

criterion_group!(benches, encoding_benchmark);
criterion_main!(benches);
