//! Run this file with `cargo test --test 02_case_insensitive_cmp`.

//! TODO: Implement a struct `CaseInsensitive`, which will allow comparing (=, <, >, etc.)
//! two (ASCII) string slices in a case insensitive way, without performing any reallocations
//! and without modifying the original strings.

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::CaseInsensitive;

    #[test]
    fn case_insensitive_same() {
        assert!(CaseInsensitive("") == CaseInsensitive(""));
        assert!(CaseInsensitive("a") == CaseInsensitive("A"));
        assert!(CaseInsensitive("a") == CaseInsensitive("a"));
        assert!(CaseInsensitive("Foo") == CaseInsensitive(&String::from("fOo")));
        assert!(CaseInsensitive("12ABBBcLPQusdaweliAS2") == CaseInsensitive("12AbbbclpQUSdawelias2"));
    }

    #[test]
    fn case_insensitive_smaller() {
        assert!(CaseInsensitive("") < CaseInsensitive("a"));
        assert!(CaseInsensitive("a") < CaseInsensitive("B"));
        assert!(CaseInsensitive("aZa") < CaseInsensitive("Zac"));
        assert!(CaseInsensitive("aZ") < CaseInsensitive("Zac"));
        assert!(CaseInsensitive("PWEasUDsx") < CaseInsensitive("PWEaszDsx"));
        assert!(CaseInsensitive("PWEasuDsx") < CaseInsensitive("PWEasZDsx"));
    }

    #[test]
    fn case_insensitive_larger() {
        assert!(CaseInsensitive("a") > CaseInsensitive(""));
        assert!(CaseInsensitive("B") > CaseInsensitive("a"));
        assert!(CaseInsensitive("Zac") > CaseInsensitive("aZa"));
        assert!(CaseInsensitive("Zac") > CaseInsensitive("aZ"));
        assert!(CaseInsensitive("PWEaszDsx") > CaseInsensitive("PWEasUDsx"));
        assert!(CaseInsensitive("PWEasZDsx") > CaseInsensitive("PWEasuDsx"));
    }
}
