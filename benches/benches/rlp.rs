//! Benchmarking alloy RLP encode and decode against parity-rlp.

use alloy_rlp::{Decodable, Encodable, RlpDecodable, RlpEncodable};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rlp::Encodable as ParityEncodable;

#[derive(RlpEncodable, RlpDecodable, rlp_derive::RlpDecodable, rlp_derive::RlpEncodable)]
pub struct MyStruct {
    pub a: u128,
    pub b: Vec<u8>,
}

fn rlp_encode(c: &mut Criterion) {
    let mut g = c.benchmark_group("rlp_encode");
    g.warm_up_time(std::time::Duration::from_secs(3));

    let my_struct = MyStruct { a: 42, b: vec![1, 2, 3, 4, 5] };

    // Alloy RLP encoding
    g.bench_with_input("rlp_encode/alloy", &my_struct, |b, my_struct| {
        b.iter(|| {
            let mut out = Vec::new();
            let _ = my_struct.encode(&mut out);
            black_box(out);
        })
    });

    // Parity RLP encoding
    g.bench_with_input("rlp_encode/parity", &my_struct, |b, my_struct| {
        b.iter(|| {
            let out = my_struct.rlp_bytes();
            black_box(out);
        })
    });
}

fn rlp_decode(c: &mut Criterion) {
    let mut g = c.benchmark_group("rlp_decode");
    g.warm_up_time(std::time::Duration::from_secs(3));

    let my_struct = MyStruct { a: 42, b: vec![1, 2, 3, 4, 5] };
    let mut encoded = Vec::new();
    let _ = my_struct.encode(&mut encoded);

    // Alloy RLP decoding
    g.bench_with_input("rlp_decode/alloy", &encoded, |b, encoded| {
        b.iter(|| {
            let decoded = MyStruct::decode(&mut encoded.as_slice()).unwrap();
            black_box(decoded);
        })
    });

    // Parity RLP decoding
    g.bench_with_input("rlp_decode/parity", &encoded, |b, encoded| {
        b.iter(|| {
            let decoded: MyStruct = rlp::decode(encoded).unwrap();
            black_box(decoded);
        })
    });
}

criterion_group!(benches, rlp_encode, rlp_decode);
criterion_main!(benches);
