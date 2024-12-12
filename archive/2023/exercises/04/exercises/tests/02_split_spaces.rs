//! Run this file with `cargo test --test 02_split_spaces`.

// Implement a struct called `SplitSpaces`, which will receive a string slice and a delimiter char
// in its constructor. The struct will then iterate over all substrings of the input, separated by
// the delimiter. The iterator should never return an empty string, it should automatically skip
// over empty strings.

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::SplitSpaces;

    #[test]
    fn split_empty() {
        let result = SplitSpaces::new("", ' ').collect::<Vec<_>>();
        assert!(result.is_empty());
    }

    #[test]
    fn split_one_delimiter() {
        let result = SplitSpaces::new("c", 'c').collect::<Vec<_>>();
        assert!(result.is_empty());
    }

    #[test]
    fn split_only_delimiters() {
        let result = SplitSpaces::new("ccc", 'c').collect::<Vec<_>>();
        assert!(result.is_empty());
    }

    #[test]
    fn split_delimiter_at_beginning() {
        let result = SplitSpaces::new(" asd", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["asd"]);
    }

    #[test]
    fn split_delimiters_at_beginning() {
        let result = SplitSpaces::new("  asd", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["asd"]);
    }

    #[test]
    fn split_delimiter_at_end() {
        let result = SplitSpaces::new("asd ", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["asd"]);
    }

    #[test]
    fn split_delimiters_at_end() {
        let result = SplitSpaces::new("asd  ", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["asd"]);
    }

    #[test]
    fn split_single_chars() {
        let result = SplitSpaces::new("a b c d e", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn split_complex() {
        let result = SplitSpaces::new("   abc   bde casdqw dee xe ", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["abc", "bde", "casdqw", "dee", "xe"]);
    }
}
