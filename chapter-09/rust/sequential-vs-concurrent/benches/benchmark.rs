use criterion::{criterion_group, criterion_main, Criterion};
use futures::future::join_all;
use std::time::Duration;
use tokio::time::sleep;

// Simulates some CPU-intensive or I/O work
async fn simulate_work() {
    sleep(Duration::from_millis(10)).await;
}

// Sequential processing of tasks
async fn process_sequential(num_tasks: usize) {
    for _ in 0..num_tasks {
        simulate_work().await;
    }
}

// Concurrent processing of tasks
async fn process_concurrent(num_tasks: usize) {
    let tasks: Vec<_> = (0..num_tasks)
        .map(|_| simulate_work())
        .collect();
    join_all(tasks).await;
}

fn concurrent_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("task_processing");
    group.sampling_mode(criterion::SamplingMode::Flat);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(2));

    for num_tasks in [10].iter() {
        group.bench_function(format!("sequential_{}_tasks", num_tasks), |b| {
            b.iter(|| rt.block_on(process_sequential(*num_tasks)));
        });

        group.bench_function(format!("concurrent_{}_tasks", num_tasks), |b| {
            b.iter(|| rt.block_on(process_concurrent(*num_tasks)));
        });
    }
    group.finish();
}

criterion_group!(benches, concurrent_benchmark);
criterion_main!(benches);