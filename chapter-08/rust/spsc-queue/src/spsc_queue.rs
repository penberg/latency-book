///! Reference:
///
///! Nhat Minh Leˆ et al. (2013) "Correct and Efficient Bounded FIFO Queues". IEEE SBAC-PAD.
use std::sync::atomic::{AtomicUsize, Ordering};

// A bounded, wait-free, single-producer, single-consumer queue.
pub struct SpscQueue<T: Default + Copy, const N: usize> {
    data: [T; N],
    front: AtomicUsize,
    back: AtomicUsize,
}

unsafe impl<T: Default + Copy, const N: usize> Sync for SpscQueue<T, N> where T: Send {}

/// A bounded, wait-free, single-producer, single-consumer queue.
impl<T: Default + Copy, const N: usize> SpscQueue<T, N> {
    /// Create a new queue.
    pub fn new() -> Self {
        // Initialize the buffer with default values.
        let data = [T::default(); N];
        let front = AtomicUsize::new(0);
        let back = AtomicUsize::new(0);
        SpscQueue { data, front, back }
    }

    /// Pushes an item into the queue. Returns an error if the queue is full.
    pub fn push(&self, value: T) -> Result<(), T> {
        let back = self.back.load(Ordering::Relaxed);
        let front = self.front.load(Ordering::Acquire);
        if front + N - back == 0 {
            return Err(value);
        }
        let ptr = self.data.as_ptr() as *mut T;
        unsafe {
            ptr.add(back % N).write(value);
        }
        self.back.store(back + 1, Ordering::Release);
        Ok(())
    }

    /// Pops an item from the queue. Returns `None` if the queue is empty.
    pub fn pop(&self) -> Option<T> {
        let front = self.front.load(Ordering::Relaxed);
        let back = self.back.load(Ordering::Acquire);
        if back - front == 0 {
            return None;
        }
        let value = self.data[front % N];
        self.front.store(front + 1, Ordering::Release);
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spsc_queue() {
        let queue = SpscQueue::<i32, 4>::new();
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.push(1), Ok(()));
        assert_eq!(queue.push(2), Ok(()));
        assert_eq!(queue.push(3), Ok(()));
        assert_eq!(queue.push(4), Ok(()));
        assert_eq!(queue.push(5), Err(5));
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(4));
        assert_eq!(queue.pop(), None);
    }
}
