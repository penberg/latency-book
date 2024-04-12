use criterion::{criterion_group, criterion_main, Criterion};
use fibonacci::{fib_iterative, fib_memoization, fib_recursive};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    group.bench_function("recursive", |b| {
        b.iter(|| {
            let _ = fib_recursive(10);
        });
    });
    group.bench_function("memoization", |b| {
        b.iter(|| {
            let _ = fib_memoization(10);
        });
    });
    group.bench_function("iterative", |b| {
        b.iter(|| {
            let _ = fib_iterative(10);
        });
    });
}
criterion_group!(benches, bench);
criterion_main!(benches);
