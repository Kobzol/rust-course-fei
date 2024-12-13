//! Run this file with `cargo test --test 08_range`.

//! TODO: write a simple data structure called `Range1D`, which represents a range of
//! 64-bit integers. Both sides of the range (start and end) are **inclusive**, e.g. a range
//! `[1, 5]` represents integers `1, 2, 3, 4, 5`.
//!
//! Implement a few basic functions:
//! - `new`: constructs a new range.
//! - `len`: returns the number of integers contained in the range.
//! - `contains`: computes whether a given point is contained in the range.
//! - `start`: returns the start of the range.
//! - `end`: returns the end of the range.
//! - `intersect`: receives another range and returns the intersection of the two ranges.
//! - `iter`: returns an immutable iterator over the integers contained in the range.
//!
//! `Range1D` should only allow representing valid ranges that are non-empty.
//! If the user attempts to create an invalid range, you should return an error from the constructor
//! itself.
//!
//! Obviously, the range should be sparse; store only the start and end values in memory, not all
//! numbers in the range :) Otherwise tests will explode.

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::Range1D;

    #[test]
    #[should_panic(expected = "Start must not be larger than end")]
    fn create_invalid_range() {
        Range1D::new(2, 1).unwrap();
    }

    #[test]
    fn create_single_item_range() {
        let range = Range1D::new(1, 1).unwrap();
        assert_eq!(range.start(), 1);
        assert_eq!(range.end(), 1);
        assert_eq!(range.len(), 1);
    }

    #[test]
    fn create_range() {
        let range = Range1D::new(1, 5).unwrap();
        assert_eq!(range.start(), 1);
        assert_eq!(range.end(), 5);
        assert_eq!(range.len(), 5);
    }

    #[test]
    fn correct_method_types() {
        let range = Range1D::new(1, 5).unwrap();
        assert_eq!(range.start(), 1u64);
        assert_eq!(range.end(), 5u64);
        assert_eq!(range.len(), 5usize);
    }

    #[test]
    fn create_range_large() {
        let range = Range1D::new(1, 50000000000000000).unwrap();
        assert_eq!(range.start(), 1);
        assert_eq!(range.end(), 50000000000000000);
        assert_eq!(range.len(), 50000000000000000);
    }

    #[test]
    fn range_copy() {
        let a = Range1D::new(1, 1).unwrap();
        let b = a;
        assert_eq!(a.start(), b.start());
    }

    #[test]
    fn range_eq() {
        let a = Range1D::new(10, 12).unwrap();
        let b = Range1D::new(10, 12).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn does_not_contain_point_before() {
        let range = Range1D::new(14, 18).unwrap();
        assert!(!range.contains(10));
        assert!(!range.contains(13));
    }

    #[test]
    fn does_not_contain_point_after() {
        let range = Range1D::new(20, 25).unwrap();
        assert!(!range.contains(26));
        assert!(!range.contains(39));
    }

    #[test]
    fn contains_point() {
        let range = Range1D::new(14, 18).unwrap();
        assert!(range.contains(14));
        assert!(range.contains(15));
        assert!(range.contains(16));
        assert!(range.contains(17));
        assert!(range.contains(18));
    }

    #[test]
    fn iterate_single() {
        let range = Range1D::new(14, 14).unwrap();
        let mut iter = range.iter();
        assert_eq!(iter.next(), Some(14));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterate_many() {
        let range = Range1D::new(20, 25).unwrap();
        let items: Vec<_> = range.iter().collect();
        assert_eq!(items, vec![20, 21, 22, 23, 24, 25]);
    }

    #[test]
    fn intersect_empty_left() {
        let a = Range1D::new(20, 25).unwrap();
        let b = Range1D::new(18, 19).unwrap();
        assert!(a.intersect(b).is_none());
    }

    #[test]
    fn intersect_empty_right() {
        let a = Range1D::new(20, 25).unwrap();
        let b = Range1D::new(28, 30).unwrap();
        assert!(a.intersect(b).is_none());
    }

    #[test]
    fn intersect_single_left() {
        let a = Range1D::new(20, 25).unwrap();
        let b = Range1D::new(18, 20).unwrap();
        assert_eq!(a.intersect(b), Some(Range1D::new(20, 20).unwrap()));
    }

    #[test]
    fn intersect_single_right() {
        let a = Range1D::new(20, 25).unwrap();
        let b = Range1D::new(25, 28).unwrap();
        assert_eq!(a.intersect(b), Some(Range1D::new(25, 25).unwrap()));
    }

    #[test]
    fn intersect_same() {
        let a = Range1D::new(20, 25).unwrap();
        let b = Range1D::new(20, 25).unwrap();
        assert_eq!(a.intersect(b), Some(Range1D::new(20, 25).unwrap()));
    }

    #[test]
    fn intersect_subset() {
        let a = Range1D::new(10, 80).unwrap();
        let b = Range1D::new(24, 38).unwrap();
        assert_eq!(a.intersect(b), Some(Range1D::new(24, 38).unwrap()));
    }

    #[test]
    fn intersect_superset() {
        let a = Range1D::new(18, 25).unwrap();
        let b = Range1D::new(4, 40).unwrap();
        assert_eq!(a.intersect(b), Some(Range1D::new(18, 25).unwrap()));
    }

    #[test]
    fn intersect_slice_left() {
        let a = Range1D::new(20, 25).unwrap();
        let b = Range1D::new(17, 21).unwrap();
        assert_eq!(a.intersect(b), Some(Range1D::new(20, 21).unwrap()));
    }

    #[test]
    fn intersect_slice_right() {
        let a = Range1D::new(20, 25).unwrap();
        let b = Range1D::new(23, 28).unwrap();
        assert_eq!(a.intersect(b), Some(Range1D::new(23, 25).unwrap()));
    }

    #[test]
    fn intersect_large() {
        let a = Range1D::new(20, 25).unwrap();
        let b = Range1D::new(23, 28).unwrap();
        assert_eq!(a.intersect(b), Some(Range1D::new(23, 25).unwrap()));
    }
}
