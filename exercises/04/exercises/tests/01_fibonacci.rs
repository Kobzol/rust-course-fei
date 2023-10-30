//! Run this file with `cargo test --test 01_fibonacci`.

// Implement a struct called `Fibonacci`, which implements `Iterator` which iterates through
// Fibonacci numbers (starting from 0).
// `Fibonacci` should implement the `Default` trait.

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::Fibonacci;

    #[test]
    fn fibonacci_first() {
        assert_eq!(Fibonacci::default().next(), Some(0));
    }

    #[test]
    fn fibonacci_ten() {
        assert_eq!(
            Fibonacci::default().take(10).collect::<Vec<_>>(),
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
        );
    }

    #[test]
    fn fibonacci_twenty() {
        assert_eq!(Fibonacci::default().skip(19).next(), Some(4181));
    }
}
