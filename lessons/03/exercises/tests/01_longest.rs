//! Run this file with `cargo test --test 01_longest`.

// TODO: Implement a function called `longest`, which will return the longer of the two
// input strings. If they are the same length, return the first string.

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::longest;

    #[test]
    fn longest_first_longer() {
        let result = longest("aqwe", "ab");
        assert_eq!(result, "aqwe");
    }

    #[test]
    fn longest_second_longer() {
        let result = longest("foo", "barbaz");
        assert_eq!(result, "barbaz");
    }

    #[test]
    fn longest_same_length() {
        let result = longest("x", "y");
        assert_eq!(result, "x");
    }

    #[test]
    fn longest_different_lifetimes_unified() {
        let a = "foo";
        let b = String::from("barx");
        let result = longest(a, &b);
        assert_eq!(result, &b);
    }

    // TODO: Can we write the `longest` function in a way that the following test compiles?
    // The function has to return one of the two input strings, and cannot copy the string data.
    // `longest` should return the first string in this case, so it should be fine to
    // drop `b`. Right? :)
    // #[test]
    // fn longest_different_lifetimes_drop() {
    //     let a = "longer";
    //     let b = String::from("short");
    //     let result = longest(a, &b);
    //     drop(b);
    //     assert_eq!(result, "longer");
    // }
}
