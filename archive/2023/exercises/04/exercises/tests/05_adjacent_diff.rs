//! Run this file with `cargo test --test 05_adjacent_diff`.

// Implement a function called `adjacent_diff`, which will receive a slice of numbers, and it will
// return an Iterator over the differences of adjacent numbers.
// E.g. `adjacent_diff(&[1, 2, 4, 1])` will iterate `1`, `2`, and `-3`.

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
