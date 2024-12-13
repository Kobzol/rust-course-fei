//! Run this file with `cargo test --test 04_iter_exercises`.

//! TODO: Implement a function called `keep_even`, which will receive something that can be turned into
//! an iterator (`IntoIterator`) of generic items, and return an iterator that will keep only the even
//! elements (so 0th element, 2nd element, etc.).
//! Try to implement the returned iterator using iterator adapters, not an explicit struct.
//! Use `impl Iterator` for the return type.

/// TODO: Implement a function called `find_third_42`, which find the index
/// of the **third** occurrence of the number 42 in the input slice.
///
/// Try to implement the function using iterator adapters.
///
/// Example 1: `[0,1,42,3,42,5,6,42,8,9]` -> Some(7)
/// Example 2: `[0,1,42,3,42,5,6,7,8,9]` -> None
fn find_third_42(vec: &[i64]) -> Option<usize> {
    todo!()
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::{find_third_42, keep_even};

    #[test]
    fn keep_even_empty_iter() {
        let iter = std::iter::empty::<u32>();
        assert_eq!(keep_even(iter).count(), 0);
    }

    #[test]
    fn keep_even_one_item() {
        let items = vec![1];
        assert_eq!(keep_even(items).collect::<Vec<_>>(), vec![1]);
    }

    #[test]
    fn keep_even_two_items() {
        let items = vec![4, 5];
        assert_eq!(keep_even(items).collect::<Vec<_>>(), vec![4]);
    }

    #[test]
    fn keep_even_more_items() {
        let items = vec![10, 15, 11, 16, 12, 17, 13, 18];
        assert_eq!(keep_even(items).collect::<Vec<_>>(), vec![10, 11, 12, 13]);
    }

    #[test]
    fn keep_even_different_iterator_type() {
        let items = vec![1, 2, 3, 4, 5].into_iter().map(|v| v * 2);
        assert_eq!(keep_even(items).collect::<Vec<_>>(), vec![2, 6, 10]);
    }

    #[test]
    fn find_third_42_empty() {
        assert!(find_third_42(&[]).is_none());
    }

    #[test]
    fn find_third_not_enough_42() {
        assert!(find_third_42(&[1, 2, 3, 2, 42, 5, 42, 8]).is_none());
    }

    #[test]
    fn find_third_three_42_begin_end() {
        assert_eq!(find_third_42(&[42, 2, 42, 2, 42]), Some(4));
    }

    #[test]
    fn find_third_three_42() {
        assert_eq!(find_third_42(&[4, 42, 2, 42, 2, 58, 42, 8]), Some(6));
    }

    #[test]
    fn find_third_all_42() {
        assert_eq!(find_third_42(&[42, 42, 42, 42, 42]), Some(2));
    }
}
