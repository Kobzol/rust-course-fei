//! Run this file with `cargo test --test 02_array_max`.

// TODO: Implement a function that finds the maximum number of an array.
// Implement it manually with a `for` cycle or `loop`.
//
// The input argument is an array of ten `i32` integers.
// How does that type look like in Rust?

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::find_largest;

    #[test]
    fn find_largest_all_same() {
        assert_eq!(find_largest([2, 2, 2, 2, 2, 2, 2, 2, 2, 2]), 2);
    }

    #[test]
    fn find_largest_increasing() {
        assert_eq!(find_largest([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 10);
    }

    #[test]
    fn find_largest_decreasing() {
        assert_eq!(find_largest([10, 9, 8, 7, 6, 5, 4, 3, 2, 1]), 10);
    }

    #[test]
    fn find_largest_random() {
        assert_eq!(find_largest([17, 10, 18, 3, 7, 8, 7, 19, 20, 8]), 20);
    }

    #[test]
    fn find_largest_negative() {
        assert_eq!(
            find_largest([-17, -10, -18, -3, -7, -8, -7, -19, -20, -8]),
            -3
        );
    }
}
