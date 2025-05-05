use alloy::{
    dyn_abi::{DynSolValue, JsonAbiExt},
    json_abi::Function,
    primitives::{uint, Address, Bytes, U256},
    sol,
    sol_types::SolCall,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ethers::{
    abi::{AbiEncode, Function as eFunction, Token},
    contract::EthCall,
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
    let mut group = c.benchmark_group("ABI Encoding");

    let json = r#"{
        "type": "function",
        "name": "swap",
        "inputs": [
        {
            "name": "amount0Out",
            "type": "uint256",
            "internalType": "uint256"
        },
        {
            "name": "amount1Out",
            "type": "uint256",
            "internalType": "uint256"
        },
        {
            "name": "to",
            "type": "address",
            "internalType": "address"
        },
        {
            "name": "data",
            "type": "bytes",
            "internalType": "bytes"
        }
        ],
        "outputs": [],
        "stateMutability": "nonpayable"
    }"#;

    group.bench_function("Ethers/Static", |b| {
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

    group.bench_function("Alloy/Static", |b| {
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

    group.bench_function("Ethers/Dynamic", |b| {
        b.iter(|| {
            let function: eFunction = serde_json::from_str(json).expect("invalid function JSON");
            let ethers_input = [
                Token::Uint(eU256::from_dec_str("100000000000000000").unwrap()),
                Token::Uint(eU256::zero()),
                Token::Address(eH160::from([0x42; 20])),
                Token::Bytes(Bytes::new().into()),
            ];

            let _ = function.encode_input(&ethers_input).unwrap();
        })
    });

    group.bench_function("Alloy/Dynamic", |b| {
        b.iter(|| {
            let func: Function = serde_json::from_str(json).unwrap();
            let input = [
                DynSolValue::Uint(uint!(100000000000000000_U256), 256),
                DynSolValue::Uint(U256::ZERO, 256),
                DynSolValue::Address(Address::from([0x42; 20])),
                DynSolValue::Bytes(Bytes::new().into()),
            ];
            let _ = func.abi_encode_input(black_box(&input)).unwrap();
        })
    });

    group.finish();
}

criterion_group!(benches, encoding_benchmark);
criterion_main!(benches);
