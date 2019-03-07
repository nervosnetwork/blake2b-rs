use blake2b_rs::blake2b;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub const S: [u8; 4096] = [1u8; 4096];

fn bench(c: &mut Criterion) {
    c.bench_function("bench_blake2b_rs", |b| {
        b.iter(|| {
            let hash = blake2_rfc::blake2b::blake2b(32, &[], &S);
            black_box(hash);
        })
    });

    c.bench_function("bench_blake2b_rfc", |b| {
        b.iter(|| {
            let mut hash = [0u8; 32];
            blake2b(&[], &S, &mut hash);
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
