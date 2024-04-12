pub fn fib_recursive(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fib_recursive(n - 1) + fib_recursive(n - 2)
}

pub fn fib_memoization(n: u64) -> u64 {
    fn fib(n: u64, cache: &mut Vec<u64>) -> u64 {
        if n <= 1 {
            return n;
        }
        if cache[n as usize] == 0 {
            let result = fib(n - 1, cache) + fib(n - 2, cache);
            cache[n as usize] = result;
        }
        cache[n as usize]
    }
    let mut cache = vec![0; (n + 1) as usize];
    fib(n, &mut cache)
}

pub fn fib_iterative(n: u64) -> u64 {
    let mut current = 0;
    let mut next = 1;
    for _ in 0..n {
        let prev = current;
        current = next;
        next = prev + next;
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_recursive() {
        assert_eq!(fib_recursive(0), 0);
        assert_eq!(fib_recursive(1), 1);
        assert_eq!(fib_recursive(2), 1);
        assert_eq!(fib_recursive(3), 2);
        assert_eq!(fib_recursive(4), 3);
        assert_eq!(fib_recursive(5), 5);
        assert_eq!(fib_recursive(6), 8);
        assert_eq!(fib_recursive(7), 13);
        assert_eq!(fib_recursive(8), 21);
        assert_eq!(fib_recursive(9), 34);
        assert_eq!(fib_recursive(10), 55);
    }

    #[test]
    fn test_fib_memoization() {
        assert_eq!(fib_memoization(0), 0);
        assert_eq!(fib_memoization(1), 1);
        assert_eq!(fib_memoization(2), 1);
        assert_eq!(fib_memoization(3), 2);
        assert_eq!(fib_memoization(4), 3);
        assert_eq!(fib_memoization(5), 5);
        assert_eq!(fib_memoization(6), 8);
        assert_eq!(fib_memoization(7), 13);
        assert_eq!(fib_memoization(8), 21);
        assert_eq!(fib_memoization(9), 34);
        assert_eq!(fib_memoization(10), 55);
    }

    #[test]
    fn test_fib_iterative() {
        assert_eq!(fib_iterative(0), 0);
        assert_eq!(fib_iterative(1), 1);
        assert_eq!(fib_iterative(2), 1);
        assert_eq!(fib_iterative(3), 2);
        assert_eq!(fib_iterative(4), 3);
        assert_eq!(fib_iterative(5), 5);
        assert_eq!(fib_iterative(6), 8);
        assert_eq!(fib_iterative(7), 13);
        assert_eq!(fib_iterative(8), 21);
        assert_eq!(fib_iterative(9), 34);
        assert_eq!(fib_iterative(10), 55);
    }
}
