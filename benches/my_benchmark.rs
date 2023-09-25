use benches_example::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("foo", |b| b.iter(|| foo()));
    c.bench_function("bar", |b| b.iter(|| bar()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
