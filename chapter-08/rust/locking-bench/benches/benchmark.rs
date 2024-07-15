use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, JoinHandle};

fn spawn_mutex_threads(
    counter: Arc<Mutex<i32>>,
    stop_signal: Arc<AtomicBool>,
    num_threads: usize,
) -> Vec<JoinHandle<()>> {
    (0..num_threads)
        .map(move |_| {
            let counter = counter.clone();
            let stop_signal = stop_signal.clone();
            thread::spawn(move || {
                while !stop_signal.load(Ordering::Relaxed) {
                    let mut counter = counter.lock().unwrap();
                    *counter += 1;
                }
            })
        })
        .collect()
}

fn spawn_rwlock_threads(
    counter: Arc<RwLock<i32>>,
    stop_signal: Arc<AtomicBool>,
    num_threads: usize,
) -> Vec<JoinHandle<()>> {
    (0..num_threads)
        .map(move |_| {
            let counter = counter.clone();
            let stop_signal = stop_signal.clone();
            thread::spawn(move || {
                while !stop_signal.load(Ordering::Relaxed) {
                    let _unused = counter.read().unwrap();
                }
            })
        })
        .collect()
}

fn stop_threads(threads: Vec<JoinHandle<()>>, stop_signal: Arc<AtomicBool>) {
    stop_signal.store(true, Ordering::Relaxed);
    for thread in threads {
        thread.join().unwrap();
    }
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("locking-bench");
    group.bench_function("Mutex::lock()+unlock() (1 thread)", |b| {
        let counter = Mutex::new(0);
        b.iter(|| {
            let mut counter = counter.lock().unwrap();
            *counter += 1;
        });
    });
    group.bench_function("Mutex::lock()+unlock() (10 threads)", |b| {
        let counter = Arc::new(Mutex::new(0));
        let stop_signal = Arc::new(AtomicBool::new(false));
        let threads = spawn_mutex_threads(counter.clone(), stop_signal.clone(), 9);
        b.iter(|| {
            let mut counter = counter.lock().unwrap();
            *counter += 1;
        });
        stop_threads(threads, stop_signal);
    });
    group.bench_function("Mutex::lock()+unlock() (100 threads)", |b| {
        let counter = Arc::new(Mutex::new(0));
        let stop_signal = Arc::new(AtomicBool::new(false));
        let threads = spawn_mutex_threads(counter.clone(), stop_signal.clone(), 99);
        b.iter(|| {
            let mut counter = counter.lock().unwrap();
            *counter += 1;
        });
        stop_threads(threads, stop_signal);
    });
    group.bench_function("RwLock::lock()+unlock() (1 thread)", |b| {
        let counter = RwLock::new(0);
        b.iter(|| {
            let _unused = counter.read().unwrap();
        });
    });
    group.bench_function("RwLock::lock()+unlock() (10 threads)", |b| {
        let counter = Arc::new(RwLock::new(0));
        let stop_signal = Arc::new(AtomicBool::new(false));
        let threads = spawn_rwlock_threads(counter.clone(), stop_signal.clone(), 9);
        b.iter(|| {
            let _unused = counter.read().unwrap();
        });
        stop_threads(threads, stop_signal);
    });
    group.bench_function("RwLock::lock()+unlock() (100 threads)", |b| {
        let counter = Arc::new(RwLock::new(0));
        let stop_signal = Arc::new(AtomicBool::new(false));
        let threads = spawn_rwlock_threads(counter.clone(), stop_signal.clone(), 99);
        b.iter(|| {
            let _unused = counter.read().unwrap();
        });
        stop_threads(threads, stop_signal);
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench
}
criterion_main!(benches);
