//! Run this file with `cargo test --test 03_merge_slices`.

// Implement a function called `merge_slices`, which is useful for the merge sort algorithm.
// It will take two sorted `u32` slices as inputs and merge them into a sorted vector (Vec).
// The function will return the vector.
// Extra: Does your function also work for a non-Copy type, e.g. String?
// Extra: Can you build a complete merge sort on top of this function? :)

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::merge_slices;

    #[test]
    fn merge_slices_empty() {
        assert_eq!(merge_slices(&[], &[]), vec![]);
    }

    #[test]
    fn merge_slices_basic() {
        assert_eq!(merge_slices(&[1, 2, 3], &[4, 5, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn merge_slices_interleaved() {
        assert_eq!(merge_slices(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn merge_slices_duplicates() {
        assert_eq!(merge_slices(&[1, 1, 3], &[1, 3, 4]), vec![1, 1, 1, 3, 3, 4]);
    }

    #[test]
    fn merge_slices_uneven_size() {
        assert_eq!(
            merge_slices(&[1, 4, 6, 8], &[0, 1, 1, 3, 4, 5, 7, 8, 9]),
            vec![0, 1, 1, 1, 3, 4, 4, 5, 6, 7, 8, 8, 9]
        );
    }

    #[test]
    fn merge_slices_first_empty() {
        assert_eq!(merge_slices(&[], &[1, 4, 8]), vec![1, 4, 8]);
    }

    #[test]
    fn merge_slices_second_empty() {
        assert_eq!(merge_slices(&[1, 9, 11], &[]), vec![1, 9, 11]);
    }
}
