//! Run this file with `cargo test --test 06_count`.

use std::collections::HashMap;

// Implement a struct called `CumulativeSum`, which will be generic over two types - a value type
// and an iterator over these value types. `CumulativeSum` will itself serve as an iterator, which
// will return a cumulative sum of the items from the input iterator.
// Try to only use iterators and no loops.
// Take a look at the `HashMap::entry` API.
fn count<'a>(items: &[&'a str]) -> HashMap<&'a str, u64> {
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::count;

    #[test]
    fn empty() {
        let result = count(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn single_each() {
        let result = count(&["foo", "bar", "baz"]);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn repeated() {
        let result = count(&[
            "foo", "bar", "foo", "baz", "bar", "foobar", "baz", "baz", "baz",
        ]);
        assert_eq!(result.len(), 4);
        assert_eq!(result.get("foo"), Some(&2));
        assert_eq!(result.get("bar"), Some(&2));
        assert_eq!(result.get("foobar"), Some(&1));
        assert_eq!(result.get("baz"), Some(&4));
    }
}
