use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(|| {
                let mut counter = counter.lock().unwrap();
                *counter += 1;
            });
        }
    });
}
