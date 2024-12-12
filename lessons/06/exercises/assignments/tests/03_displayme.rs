//! Run this file with `cargo test --test 03_displayme`.

//! TODO: implement a procedural derive macro called `DisplayMe`, which will implement the
//! `Display` trait for the target crate.
//! The macro is located in `displayme/src/lib.rs`, see the description there for more information.
//!
//! You can run `cargo expand --package week06 --test 03_displayme` to examine the expanded output
//! after macros are applied.
#![allow(unused)]

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use displayme::DisplayMe;

    #[test]
    fn display_unit() {
        #[derive(DisplayMe)]
        struct Foo;
        assert_eq!(format!("{}", Foo), r#"struct Foo;"#.to_string());
    }

    #[test]
    fn display_empty() {
        #[derive(DisplayMe)]
        struct Foo {}
        assert_eq!(format!("{}", Foo {}), r#"struct Foo {}"#.to_string());
    }

    #[test]
    fn display_tuple_struct() {
        #[derive(DisplayMe)]
        struct Bar(bool, u32, String);
        assert_eq!(
            format!("{}", Bar(true, 42, "foo".to_string())),
            r#"struct Bar (
    0: true,
    1: 42,
    2: foo
)"#
            .to_string()
        );
    }

    #[test]
    fn display_named() {
        #[derive(DisplayMe)]
        struct Foo {
            a: u32,
            b: u32,
        }
        assert_eq!(
            format!("{}", Foo { a: 5, b: 6 }),
            r#"struct Foo {
    a: 5,
    b: 6
}"#
            .to_string()
        );
    }
}
