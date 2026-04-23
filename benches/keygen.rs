use age_setup::build_keypair;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_build_keypair(c: &mut Criterion) {
    c.bench_function("build_keypair", |b| b.iter(|| build_keypair().unwrap()));
}

criterion_group!(benches, bench_build_keypair);
criterion_main!(benches);
