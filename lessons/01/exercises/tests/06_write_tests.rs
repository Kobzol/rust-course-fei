//! Run this file with `cargo test --test 06_write_tests`.

/// This function implements a string sanitization logic that should uphold the following
/// properties:
/// - After sanitization, the result must not end with the character `x`
/// - After sanitization, the result must not end with the character `o`
/// - After sanitization, the result must not end with the string `.exe`
///
/// The function assumes that the input to the function only consists of lower and uppercase
/// characters from the English alphabet and digits 0-9.
///
/// The implementation contains some bugs.
///
/// Your task is to write a set (at least 8) of unit tests, use them to find (at least 2) bugs in
/// this function and then fix the function.
fn sanitize(input: &str) -> &str {
    // Remove all x from the end of the string
    let input = input.trim_end_matches("x");
    // Remove all o from the end of the string
    let mut input = input.trim_end_matches("o");

    // Remove .exe from the end
    if input.ends_with(".exe") {
        input = &input[0..input.len() - 4];
    }

    input
}

/// TODO: write tests for the `sanitize` function
///
/// Bonus: can you find any bugs using the [proptest](https://proptest-rs.github.io/proptest/intro.html)
/// crate? There is a commented-out example at the bottom of the `tests` module; uncomment it to
/// get started.
#[cfg(test)]
mod tests {
    use super::sanitize;

    // To solve the bonus task, uncomment the `proptest` dependency in `Cargo.toml` and the
    // following example. Replace the sample property with your own properties. The explicit
    // `failure_persistence` configuration stores failing seeds in
    // `tests/06_write_tests.failures`, so they are replayed on the next run.
    //
    // use proptest::prelude::*;
    // use proptest::test_runner::FileFailurePersistence;
    //
    // proptest! {
    //     #![proptest_config(ProptestConfig {
    //         failure_persistence: Some(Box::new(
    //             FileFailurePersistence::WithSource("failures"),
    //         )),
    //         ..ProptestConfig::default()
    //     })]
    //
    //     #[test]
    //     fn sanitize_does_not_panic(s in "[a-zA-Z0-9]*") {
    //         let _ = sanitize(&s);
    //     }
    // }
}
