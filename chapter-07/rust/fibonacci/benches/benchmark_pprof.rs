use criterion::{criterion_group, criterion_main, Criterion};
use fibonacci::{fib_iterative, fib_memoization, fib_recursive};
use pprof::criterion::{Output, PProfProfiler};

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

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench
}
criterion_main!(benches);