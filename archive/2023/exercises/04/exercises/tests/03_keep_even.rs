//! Run this file with `cargo test --test 03_keep_even`.

// Implement a function called `keep_even`, which will receive something that can be turned into
// an iterator (`IntoIterator`) of generic items, and return an iterator that will keep only the even
// elements (so 0th element, 2nd element, etc.).
// Try to implement the returned iterator using iterator adapters, not an explicit struct.
// Use `impl Iterator` for the return type.

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::keep_even;

    #[test]
    fn keep_even_empty() {
        let iter = std::iter::empty::<u32>();
        assert_eq!(keep_even(iter).count(), 0);
    }

    #[test]
    fn keep_even_one() {
        let items = vec![1];
        assert_eq!(keep_even(items).collect::<Vec<_>>(), vec![1]);
    }

    #[test]
    fn keep_even_two() {
        let items = vec![4, 5];
        assert_eq!(keep_even(items).collect::<Vec<_>>(), vec![4]);
    }

    #[test]
    fn keep_even_more() {
        let items = vec![10, 15, 11, 16, 12, 17, 13, 18];
        assert_eq!(keep_even(items).collect::<Vec<_>>(), vec![10, 11, 12, 13]);
    }
}
