//! Run this file with `cargo test --test 01_longest`.

// Implement a function called `longest`, which will return the longer of the two
// input strings. If they are the same length, return the first string.

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::longest;

    #[test]
    fn longest_basic() {
        let result = longest("a", "ab");
        assert_eq!(result, "ab");
    }

    #[test]
    fn longest_same_length() {
        let result = longest("a", "b");
        assert_eq!(result, "a");
    }

    #[test]
    fn longest_different_lifetimes_unified() {
        let a = "foo";
        let b = String::from("barx");
        let result = longest(a, &b);
        assert_eq!(result, &b);
    }

    // Can we write `longest` in a way that this test will compile?
    // `longest` should return the first string, so it should be fine to drop it.
    // #[test]
    // fn longest_different_lifetimes_different() {
    //     let a = "longer";
    //     let b = String::from("short");
    //     let result = longest(a, &b);
    //     drop(b);
    //     assert_eq!(result, "longer");
    // }
}
