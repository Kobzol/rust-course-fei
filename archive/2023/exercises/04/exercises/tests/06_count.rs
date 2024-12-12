//! Run this file with `cargo test --test 06_count`.

use std::collections::HashMap;

// Implement a function called `count`, which will receive a slice of strings, and it will return
// a hash map that will count the number of occurrences of each string. Try to avoid allocating
// any strings in the function.
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
