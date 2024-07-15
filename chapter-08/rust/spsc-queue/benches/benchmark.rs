use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use spsc_queue::queue::Queue;
use spsc_queue::spsc_queue::SpscQueue;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn bench(c: &mut Criterion) {
    queue_bench(c);
    spsc_bench(c);
}

fn queue_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sync-queue-bench");
    group.bench_function("Queue::pop()", |b| {
        let consumer_queue = Arc::new(Mutex::new(Queue::<i32, 128>::new()));
        let producer_queue = consumer_queue.clone();
        let producer_thread = thread::spawn(move || {
            for i in 0..128 {
                producer_queue.lock().unwrap().push(i).unwrap();
            }
        });
        b.iter(|| {
            for _ in 0..128 {
                consumer_queue.lock().unwrap().pop();
            }
        });
        producer_thread.join().unwrap();
    });
}

fn spsc_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("spsc-queue-bench");
    group.bench_function("SpscQueue::pop()", |b| {
        let consumer_queue = Arc::new(SpscQueue::<i32, 128>::new());
        let producer_queue = consumer_queue.clone();
        let producer_thread = thread::spawn(move || {
            for i in 0..128 {
                producer_queue.push(i).unwrap();
            }
        });
        b.iter(|| {
            for _ in 0..128 {
                consumer_queue.pop();
            }
        });
        producer_thread.join().unwrap();
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench
}
criterion_main!(benches);
