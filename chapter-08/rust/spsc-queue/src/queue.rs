/// A bounded queue.
pub struct Queue<T: Default + Copy, const N: usize> {
    data: [T; N],
    front: usize,
    back: usize,
}

impl<T: Default + Copy, const N: usize> Queue<T, N> {
    pub fn new() -> Self {
        let data = [T::default(); N];
        let front = 0;
        let back = 0;
        Queue { data, front, back }
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.front + N - self.back == 0 {
            return Err(value);
        }
        self.data[self.back % N] = value;
        self.back += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.back - self.front == 0 {
            return None;
        }
        let value = self.data[self.front % N];
        self.front += 1;
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_queue() {
        let mut queue = Queue::<i32, 4>::new();
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
