//! Run this file with `cargo test --test 04_match_parentheses`.

// Implement a function called `match_parentheses`.
// It will receive a string containing arbitrary characters.
// Check that all parentheses within the string (`()`, `[]`, `{}`) are matched properly, i.e. that
// `(` precedes `)`, there is an even number of opening and closing parentheses and that the
// parentheses are not mismatched (e.g. `(` followed by `]`).
// If everything is matched properly, return `true`, otherwise return `false`.
// Hint: there is a useful data structure that can be used for this.

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::match_parentheses;

    #[test]
    fn match_parentheses_empty() {
        assert!(match_parentheses(""));
        assert!(match_parentheses("foobar"));
    }

    #[test]
    fn match_parentheses_wrong_order() {
        assert!(!match_parentheses(")("));
    }

    #[test]
    fn match_parentheses_leftover() {
        assert!(!match_parentheses("("));
        assert!(!match_parentheses("[]("));
        assert!(!match_parentheses("([]"));
    }

    #[test]
    fn match_parentheses_extra_closing() {
        assert!(!match_parentheses(")"));
        assert!(!match_parentheses("[])"));
        assert!(!match_parentheses("x([{)}])y"));
    }

    #[test]
    fn match_parentheses_wrong_matched_type() {
        assert!(!match_parentheses("[)"));
        assert!(!match_parentheses("([})"));
        assert!(!match_parentheses("(((([}))))"));
    }
}
