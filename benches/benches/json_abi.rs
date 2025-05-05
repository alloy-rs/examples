//! Benchmarks alloy JSON-ABI serialization and deserialization against ethabi.
#![allow(unknown_lints, clippy::incompatible_msrv)]

use alloy::json_abi::{Function, JsonAbi};
use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use std::{hint::black_box, time::Duration};

fn ser_group(c: &mut Criterion) {
    let mut g = c.benchmark_group("JSON-ABI Serialization");
    g.warm_up_time(Duration::from_secs(3));
    ser(&mut g, "Seaport", include_str!("../artifacts/Seaport.json"));
    ser(&mut g, "PoolManager", include_str!("../artifacts/UniV4PoolManager.json"));
    ser(&mut g, "UniswapV3Pool", include_str!("../artifacts/UniswapV3Pool.json"));
}

fn deser_group(c: &mut Criterion) {
    let mut g = c.benchmark_group("JSON-ABI Deserialization");
    g.warm_up_time(Duration::from_secs(3));
    deser(&mut g, "Seaport", include_str!("../artifacts/Seaport.json"));
    deser(&mut g, "PoolManager", include_str!("../artifacts/UniV4PoolManager.json"));
    deser(&mut g, "UniswapV3Pool", include_str!("../artifacts/UniswapV3Pool.json"));
}

fn deser(g: &mut BenchmarkGroup<'_, WallTime>, name: &str, s: &str) {
    type A = JsonAbi;
    type E = ethabi::Contract;

    g.bench_function(format!("EthAbi/{name}"), |b| {
        b.iter(|| -> E { serde_json::from_str(black_box(s)).unwrap() });
    });

    g.bench_function(format!("Alloy/{name}"), |b| {
        b.iter(|| -> A { serde_json::from_str(black_box(s)).unwrap() });
    });
}

fn ser(g: &mut BenchmarkGroup<'_, WallTime>, name: &str, s: &str) {
    type A = JsonAbi;
    type E = ethabi::Contract;

    g.bench_function(format!("EthAbi/{name}"), |b| {
        let abi = serde_json::from_str::<E>(s).unwrap();
        b.iter(|| serde_json::to_string(black_box(&abi)).unwrap());
    });

    g.bench_function(format!("Alloy/{name}"), |b| {
        let abi = serde_json::from_str::<A>(s).unwrap();
        b.iter(|| serde_json::to_string(black_box(&abi)).unwrap());
    });
}

fn signature(c: &mut Criterion) {
    let mut g = c.benchmark_group("Serde Function Signature");
    g.warm_up_time(Duration::from_secs(1));
    serde_signature(&mut g, include_str!("../artifacts/LargeFunction.json"));
}

fn serde_signature(g: &mut BenchmarkGroup<'_, WallTime>, s: &str) {
    let mut alloy = serde_json::from_str::<Function>(s).unwrap();
    let mut ethabi = serde_json::from_str::<ethabi::Function>(s).unwrap();

    assert_eq!(alloy.selector(), ethabi.short_signature());

    // clear outputs so ethabi doesn't format them
    alloy.outputs.clear();
    ethabi.outputs.clear();

    assert_eq!(alloy.selector(), ethabi.short_signature());
    assert_eq!(alloy.signature(), ethabi.signature());

    g.bench_function("EthAbi/Serialize", |b| {
        b.iter(|| black_box(&ethabi).signature());
    });

    g.bench_function("Alloy/Serialize", |b| {
        b.iter(|| black_box(&alloy).signature());
    });

    g.bench_with_input("EthAbi/Deserialize", s, |b, s| {
        b.iter(|| {
            let _f: ethabi::Function = serde_json::from_str(black_box(s)).unwrap();
        });
    });

    g.bench_with_input("Alloy/Deserialize", s, |b, s| {
        b.iter(|| {
            let _f: Function = serde_json::from_str(black_box(s)).unwrap();
        });
    });
}

criterion_group!(benches, ser_group, deser_group, signature);
criterion_main!(benches);
