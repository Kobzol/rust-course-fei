//! Run this file with `cargo test --test 05_adjacent_diff`.

// Implement a struct called `CumulativeSum`, which will be generic over two types - a value type
// and an iterator over these value types. `CumulativeSum` will itself serve as an iterator, which
// will return a cumulative sum of the items from the input iterator.
// E.g. `CumulativeSum::new(vec![1, 2, 3])` will iterate `1`, `3`, and `6`.

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
