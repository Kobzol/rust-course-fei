//! Run this file with `cargo test --test 02_match_exercises`.

// TODO: Implement the following very simple match exercises.
// Use pattern matching (the `match` expression) to implement the functions.

/// Return 1 if `v` is either 1, 3 or 5.
/// Return 2 if `v` is either 0, 2 or 4.
/// Return 3 in any other situation.
/// The match expression should have three arms.
fn exercise_1(v: u8) -> u8 {
}

/// Return 1 if `v` is between 0 (inclusive) and 100 (exclusive).
/// Return 2 if `v` is between 100 (inclusive) and 200 (exclusive).
/// Return 3 in any other situation.
/// The match expression should have three arms.
fn exercise_2(v: u8) -> u8 {
}

/// Return 1 if `v` is in the first half of the English alphabet (`a-n` or `A-N`) (uppercase *or* lowercase).
/// Return 2 if `v` is in the second half of the English alphabet (`o-z` or `O-Z`) (uppercase *or* lowercase).
/// Return 3 if `v` is not in the English alphabet.
/// The match expression should have three arms.
fn exercise_3(v: char) -> u8 {
}

/// Check if the character in `v` is a digit (0-9).
/// The match expression should have two arms.
fn exercise_4(v: char) -> bool {
}

/// Check if `v` is a digit. If it is, return a u32 containing the numerical value of that digit
/// (wrapped in `Some`).
/// Use a match expression.
///
/// Hint: it may help to cast a char to a number (`v as u32`) to solve this problem.
/// In the standard library, there is `char::to_digit() to solve this exact problem.
/// Try not to use its implementation, but if you're stuck, use it for inspiration
/// and maybe to check your solution.
///
/// If `v` is not a digit, return `None`.
fn exercise_5(v: char) -> Option<u32> {
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::{exercise_1, exercise_2, exercise_3, exercise_4, exercise_5};

    #[test]
    fn test_exercise_1() {
        assert_eq!(exercise_1(1), 1);
        assert_eq!(exercise_1(3), 1);
        assert_eq!(exercise_1(5), 1);
        assert_eq!(exercise_1(0), 2);
        assert_eq!(exercise_1(2), 2);
        assert_eq!(exercise_1(4), 2);

        for i in 0..=255u8 {
            if i > 5 {
                assert_eq!(exercise_1(i), 3);
            }
        }
    }

    #[test]
    fn test_exercise_2() {
        for i in 0..100u8 {
            assert_eq!(exercise_2(i), 1);
        }
        for i in 100..200u8 {
            assert_eq!(exercise_2(i), 2);
        }
        for i in 200..=255 {
            assert_eq!(exercise_2(i), 3);
        }
    }

    #[test]
    fn test_exercise_3() {
        for i in 'a'..='n' {
            assert_eq!(exercise_3(i), 1);
        }
        for i in 'o'..='z' {
            assert_eq!(exercise_3(i), 2);
        }
        for i in 'A'..='N' {
            assert_eq!(exercise_3(i), 1);
        }
        for i in 'O'..='Z' {
            assert_eq!(exercise_3(i), 2);
        }
        for i in '0'..='9' {
            assert_eq!(exercise_3(i), 3);
        }
    }

    #[test]
    fn test_exercise_4() {
        for i in '0'..='9' {
            assert!(exercise_4(i));
        }
        for i in 'A'..='z' {
            assert!(!exercise_4(i));
        }
    }

    #[test]
    fn test_exercise_5() {
        for i in '0'..='z' {
            assert_eq!(exercise_5(i), i.to_digit(10));
        }
    }
}
