//! Run this file with `cargo test --test 04_adjacent_diff`.

//! TODO: Implement a function called `adjacent_diff`, which will receive a slice of numbers, and it will
//! return an Iterator over the differences of adjacent numbers.
//! E.g. `adjacent_diff(&[1, 2, 4, 1])` will iterate `1`, `2`, and `-3`.
//!
//! Try to implement the iterator directly within the function, using various iterator combinators.
//! Do not create a separate struct that would implement the `Iterator` trait.
//!
//! The iterator has to be **lazy**! It should not copy the whole input array
//! (in other words, it should have space complexity O(1)).

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::adjacent_diff;

    #[test]
    fn empty() {
        assert_eq!(adjacent_diff(&[]).count(), 0);
    }

    #[test]
    fn single_item() {
        assert_eq!(adjacent_diff(&[1]).count(), 0);
    }

    #[test]
    fn two_items() {
        assert_eq!(adjacent_diff(&[1, 3]).collect::<Vec<_>>(), vec![2]);
    }

    #[test]
    fn many_items() {
        assert_eq!(
            adjacent_diff(&[1, 3, 2, 4, 8, 12, 5, 10]).collect::<Vec<_>>(),
            vec![2, -1, 2, 4, 4, -7, 5]
        );
    }
}
