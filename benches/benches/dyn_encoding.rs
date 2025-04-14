use alloy::{
    dyn_abi::{DynSolValue, JsonAbiExt},
    json_abi::Function,
    primitives::{uint, Address, Bytes, U256},
    sol,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ethers::{
    abi::{Function as eFunction, Token},
    types::{H160 as eH160, U256 as eU256},
};

sol! {
    #[sol(rpc)]
    function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external;
}

fn encoding_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Dynamic");

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

    group.bench_function("Ethers", |b| {
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

    group.bench_function("Alloy", |b| {
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
