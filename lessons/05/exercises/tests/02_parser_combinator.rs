//! Run this file with `cargo test --test 02_parser_combinator`.

//! TODO: Implement a JSON parser using parser combinators.
//! Good news: I wrote the parser for you.
//! Bad news: You have to implement the parser combinators (the building LEGO blocks that were used
//! to implement the JSON parser).
//! But don't worry, even though this looks difficult, it's actually quite easy.
//! After the first few combinators, you'll get the hang of it.
//!
//! Parser combinators are one of the approaches for building parsers.
//! In this case, parsers are functions that receive a string and return a parsed
//! value and the rest of the unparsed input.
//! A parser combinator is then a function that returns a parser.
//! Combinators can receive other parsers as arguments, this can be used to create complex parsers.
//!
//! Implement the following parser combinators:
//! - `take_while`: Returns a parser that eats characters accepted by the passed function, and
//!    joins them to a string.
//! - `string_parser`: Returns a parser that parses a specific string.
//! - `map`: Receives a parser and a mapping function, and applies that mapping function to the
//!   parsed value of the parser if it succeeds.
//! - `try_map`: Receives a parser and a mapping function, and applies that mapping function to the
//!   parsed value of the parser if it succeeds. The mapping function should return
//!   `Result<T, String>`; if it returns an error, the parser should fail.
//! - `or`: Receives two parsers and returns a parser that accepts the first parser out of these
//!   two that succeeds.
//! - `opt`: Receives a parser and returns a new parser that returns `Option<T>`. If the received
//!   parser would fail, it instead succeeds while returning `None`.
//! - `preceded_by`: Receives two parsers, P1 and P2, and returns a parser that first checks
//!   that the input begins with P1, and then is followed by P2. The return value of P1 is thrown
//!   away.
//! - `followed_by`: Receives two parsers, P1 and P2, and returns a parser that first checks
//!   that the input begins with P1, and then is followed by P2. The return value of P2 is thrown
//!   away.
//! - `delimited_by`: Receives two characters (A and B) and a parser P, and returns a parser that
//!   only succeeds if the input starts with A, then successfully parses P, and is then followed by
//!   B. Try to implement this combinator using a combination of `preceded_by` and `followed_by`.
//! - `sequence`: Receives two parsers A and B and returns a parser that checks that the input
//!   begins with A and is then followed by B. If it succeeds, it returns the return values of A and B
//!   in a tuple.
//! - `repeated`: Receives a parser and returns a parser that tries to apply it as many times as
//!   possible. It collects all the return values in a `Vec`.
//!
//! And just so that you don't feel sad that you only build the low-level blocks and not a full
//! parser, also implement a simple parser of HTML elements called `tag_parser`. It will receive
//! a name of a tag, and then parse the text contents inside `<tag>...</tag>`. The contents
//! within an element can be any characters except for `<`.
//!
//! See tests for details on how should the individual parser combinators work!
//! Note that parsers only succeed if they find what they need at the **beginning** of the input.
//!
//! Hint #1: If you run into issues with the returned type of a parser combinator, try to use
//! `FnMut` instead of `Fn`.
//! Hint #2: Use `impl FnMut(...) -> ...` as the return type of parser combinators instead of
//! using `impl Parser`. This is essentially a current limitation of the Rust type system.
//!
//! **Important**: you are supposed to implement the combinators manually. Do not use an external
//! crate for it, such as nom, winnow or chumsky.

/// Represents a parser that returns value of type `T`.
/// A parser takes a string and returns `ParseResult`.
/// If it succeeds with parsing, it returns the rest of the input string after the part
/// that was parsed, along with the parsed value of `T`.
/// If it fails, it returns an error described by a `String`.
trait Parser<T> {
    fn parse<'a>(&mut self, input: &'a str) -> ParseResult<&'a str, T>;
}

/// For simplicity, we implement this trait for all functions that receive a string and return
/// `ParseResult`. Therefore, we can call `<function>.parse(input)`.
impl<F, T> Parser<T> for F
where
    F: FnMut(&str) -> ParseResult<&str, T>,
{
    fn parse<'a>(&mut self, input: &'a str) -> ParseResult<&'a str, T> {
        self(input)
    }
}

/// Return value of a parser. `I` is essentially always `&str`, while `T` is the parsed
/// value of the parser.
/// `String` is used to represent an error.
type ParseResult<I, T> = Result<(I, T), String>;

/// This is the simplest parser combinator.
/// It returns a parser that parses a specific character.
fn char_parser(c: char) -> impl Fn(&str) -> ParseResult<&str, char> {
    // We need `move`, because we use an external value (`c`).
    // We cannot just reference it, because `c` disappears when this function
    // returns, but the returned closure will be used later.
    move |input: &str| {
        let Some(rest) = input.strip_prefix(c) else {
            return Err(format!("Input does not begin with {c}"));
        };
        Ok((rest, c))
    }
}


/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::{char_parser, ParseResult, Parser};

    use std::collections::HashMap;
    use std::fmt::Debug;

    #[test]
    fn test_char() {
        let parser = |c| char_parser(c);
        check(parser('c'), "c", "", 'c');
        check(parser('1'), "1b", "b", '1');
        check_fail(parser('c'), "x");
    }

    #[test]
    fn test_take_while() {
        use super::take_while;

        check(take_while(|c| c.is_ascii_digit()), "c", "c", String::new());
        check(
            take_while(|c| c.is_uppercase()),
            "ABc",
            "c",
            String::from("AB"),
        );
        check(
            take_while(|c| c.is_ascii_digit()),
            "123",
            "",
            String::from("123"),
        );
        check(take_while(|_| true), "12č3", "", String::from("12č3"));
    }

    #[test]
    fn test_string() {
        use super::string_parser;

        check_fail(string_parser("foo"), "");
        check_fail(string_parser("foo"), "bar");
        check(string_parser("ABC"), "ABC", "", String::from("ABC"));
        check(string_parser("ABC"), "ABCx", "x", String::from("ABC"));
        check(
            string_parser("čau, jak se máš?"),
            "čau, jak se máš? dobře",
            " dobře",
            String::from("čau, jak se máš?"),
        );
        check_fail(string_parser("ABC"), "ABDC");
        check_fail(string_parser("čau, jak se máš?"), "čau, jak se máš");
    }

    #[test]
    fn test_map() {
        use super::{map, string_parser};

        let parser = map(string_parser("foo"), |res| {
            assert_eq!(res, "foo".to_string());
            res.to_uppercase()
        });
        check(parser, "foobar", "bar", "FOO".to_string());

        let parser = map(string_parser("foo"), |_| 1);
        check_fail(parser, "bar");
    }

    #[test]
    fn test_or() {
        use super::{map, or, string_parser};

        let p1 = || map(string_parser("foo"), |res| res.to_uppercase());
        let p2 = || string_parser("bar");
        check(or(p1(), p2()), "foobar", "bar", "FOO".to_string());
        check(or(p1(), p2()), "barfoo", "foo", "bar".to_string());
        check(
            or(
                string_parser("foo"),
                map(string_parser("foo"), |s| s.to_uppercase()),
            ),
            "foo",
            "",
            "foo".to_string(),
        );
        check_fail(or(p1(), p2()), "xbarfoo");
    }

    #[test]
    fn test_opt() {
        use super::{opt, string_parser};

        let parser = || opt(string_parser("foo"));
        check(parser(), "foobar", "bar", Some("foo".to_string()));
        check(parser(), "bar", "bar", None);
    }

    #[test]
    fn test_preceded_by() {
        use super::{preceded_by, string_parser};

        let parser = || preceded_by(string_parser("foo"), string_parser("bar"));
        check(parser(), "foobar", "", "bar".to_string());
        check(
            preceded_by(char_parser('c'), string_parser("bar")),
            "cbarx",
            "x",
            "bar".to_string(),
        );
        check_fail(parser(), "fobar");
    }

    #[test]
    fn test_followed_by() {
        use super::{followed_by, map, string_parser};

        let parser = || {
            followed_by(
                string_parser("foo"),
                map(string_parser("bar"), |f| f.to_uppercase()),
            )
        };
        check(parser(), "foobar", "", "foo".to_string());
        check(
            followed_by(string_parser("bar"), char_parser('c')),
            "barc",
            "",
            "bar".to_string(),
        );
        check_fail(parser(), "fobar");
    }

    #[test]
    fn test_delimited_by() {
        use super::{delimited_by, string_parser};

        check(
            delimited_by('a', 'b', string_parser("foo")),
            "afoobx",
            "x",
            "foo".to_string(),
        );
        check_fail(delimited_by('a', 'b', string_parser("foo")), "afoox");
    }

    #[test]
    fn test_sequence() {
        use super::{char_parser, map, sequence, string_parser, take_while};

        check(sequence(char_parser('a'), char_parser('b')), "abc", "c", ('a', 'b'));
        check(sequence(string_parser("hello"), char_parser('!')),
              "hello!world", "world", (String::from("hello"), '!'));
        check(sequence(take_while(|c| c.is_ascii_digit()), char_parser('a')),
              "123a4", "4", (String::from("123"), 'a'));
        check(sequence(take_while(|c| c.is_ascii_alphabetic()), char_parser('!')),
              "abc!xyz", "xyz", (String::from("abc"), '!'));
        check(sequence(char_parser('x'), take_while(|c| c.is_ascii_lowercase())),
              "xhello123", "123", ('x', String::from("hello")));
        check(sequence(string_parser("begin"), take_while(|c| c.is_ascii_alphabetic())),
              "beginABC123", "123", (String::from("begin"), String::from("ABC")));
        check(sequence(
            map(char_parser('1'), |_| 42),
            map(char_parser('2'), |_| 84)),
              "12x", "x", (42, 84));
        check(sequence(
            take_while(|c| c.is_uppercase()),
            take_while(|c| c.is_ascii_digit())),
              "HELLO123abc", "abc", (String::from("HELLO"), String::from("123")));
    }

    #[test]
    fn test_repeated() {
        use super::{char_parser, delimited_by, followed_by, map, repeated, take_while};

        check(repeated(char_parser('a')), "aaab", "b", vec!['a', 'a', 'a']);
        check(repeated(char_parser('x')), "yyy", "yyy", Vec::<char>::new());
        check(repeated(map(char_parser('1'), |c| c.to_string())), "1112", "2",
              vec![String::from("1"), String::from("1"), String::from("1")]);
        check(repeated(delimited_by('[', ']', char_parser('x'))),
              "[x][x]a", "a", vec!['x', 'x']);
        check(repeated(followed_by(char_parser('a'), char_parser('b'))),
              "ababac", "ac", vec!['a', 'a']);
        check(repeated(map(char_parser('1'), |_| 1)),
              "111x", "x", vec![1, 1, 1]);
        check(
            repeated(map(
                delimited_by('(', ')', take_while(|c| c.is_ascii_digit())),
                |c| c.to_string())),
            "(12)(34)A", "A", vec![String::from("12"), String::from("34")]);
    }

    #[test]
    fn test_html_tag_parser() {
        use super::tag_parser;

        check(tag_parser("div"), "<div></div>", "", "".to_string());
        check(
            tag_parser("div"),
            "<div>ahoj</div>x",
            "x",
            "ahoj".to_string(),
        );
        check(
            tag_parser("div"),
            "<div>ahoj</div>x",
            "x",
            "ahoj".to_string(),
        );
        check(
            tag_parser("span"),
            "<span>čau</span>",
            "",
            "čau".to_string(),
        );
    }

    #[test]
    fn test_html_tag_parser_unclosed() {
        use super::tag_parser;

        check_fail(tag_parser("span"), "<");
        check_fail(tag_parser("span"), "<span");
        check_fail(tag_parser("span"), "<span>");
        check_fail(tag_parser("span"), "<span>>");
        check_fail(tag_parser("span"), "<span><span>");
        check_fail(tag_parser("span"), "<span>asd< /span>");
        check_fail(tag_parser("span"), "<span>asd< /span>");
        check_fail(tag_parser("span"), "<span>asd</span");
    }

    #[test]
    fn json_bool() {
        check(json_parser(), "     true", "", Json::Bool(true));
        check(json_parser(), "\nfalse", "", Json::Bool(false));
        check(json_parser(), "false\nx", "\nx", Json::Bool(false));
        check(json_parser(), "true", "", Json::Bool(true));
        check(json_parser(), "false", "", Json::Bool(false));
        check(json_parser(), "falsetrue", "true", Json::Bool(false));
        check_fail(json_parser(), "xyz");
        check_fail(json_parser(), "tru");
        check_fail(json_parser(), "fals");
    }

    #[test]
    fn json_integer() {
        check(json_parser(), "5", "", Json::Integer(5));
        check(json_parser(), "   523", "", Json::Integer(523));
        check(json_parser(), "123", "", Json::Integer(123));
        check(json_parser(), "859xyz", "xyz", Json::Integer(859));
        check_fail(json_parser(), "abcd");
        check_fail(json_parser(), "abcd123");
        check_fail(json_parser(), "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111");
    }

    #[test]
    fn json_string() {
        check(
            json_parser(),
            r#""foo""#,
            "",
            Json::String("foo".to_string()),
        );
        check(
            json_parser(),
            r#"    "foobar""#,
            "",
            Json::String("foobar".to_string()),
        );
        check(
            json_parser(),
            r#""foobar"x"#,
            "x",
            Json::String("foobar".to_string()),
        );
        check_fail(json_parser(), r#""foo"#);
        check_fail(json_parser(), r#"foo""#);
        check_fail(json_parser(), "foo");
    }

    #[test]
    fn json_array() {
        check(json_parser(), r#"[]"#, "", Json::Array(vec![]));
        check(
            json_parser(),
            r#"[1,2,3]"#,
            "",
            Json::Array(vec![Json::Integer(1), Json::Integer(2), Json::Integer(3)]),
        );
        check(
            json_parser(),
            r#"    [    1   ,  2  , 3    ]"#,
            "",
            Json::Array(vec![Json::Integer(1), Json::Integer(2), Json::Integer(3)]),
        );
        check(
            json_parser(),
            r#"[true, false, null, "foobar", [true, 2]]"#,
            "",
            Json::Array(vec![
                Json::Bool(true),
                Json::Bool(false),
                Json::Null,
                Json::String("foobar".to_string()),
                Json::Array(vec![Json::Bool(true), Json::Integer(2)]),
            ]),
        );
        check(
            json_parser(),
            r#"[true, false, null, "foobar", [true, 2]]"#,
            "",
            Json::Array(vec![
                Json::Bool(true),
                Json::Bool(false),
                Json::Null,
                Json::String("foobar".to_string()),
                Json::Array(vec![Json::Bool(true), Json::Integer(2)]),
            ]),
        );
        check_fail(json_parser(), "[");
        check_fail(json_parser(), "[1,,]");
        check_fail(json_parser(), "[,1]");
    }

    #[test]
    fn json_object() {
        check(json_parser(), r#"{}"#, "", Json::Object(Default::default()));
        check(
            json_parser(),
            r#"{"1":2}"#,
            "",
            Json::Object({
                let mut map = HashMap::new();
                map.insert("1".to_string(), Json::Integer(2));
                map
            }),
        );
        check(
            json_parser(),
            r#"     {      "1"    :   2   }"#,
            "",
            Json::Object({
                let mut map = HashMap::new();
                map.insert("1".to_string(), Json::Integer(2));
                map
            }),
        );
        check(
            json_parser(),
            r#"{"abc": [1, 2, true], "foo": {"a": 5}}"#,
            "",
            Json::Object({
                let mut map = HashMap::new();
                map.insert(
                    "abc".to_string(),
                    Json::Array(vec![Json::Integer(1), Json::Integer(2), Json::Bool(true)]),
                );
                map.insert(
                    "foo".to_string(),
                    Json::Object({
                        let mut map = HashMap::new();
                        map.insert("a".to_string(), Json::Integer(5));
                        map
                    }),
                );
                map
            }),
        );
        check_fail(json_parser(), r#"{1:2}"#);
    }

    /// Simplified JSON, without floating point numbers
    #[derive(Debug, Eq, PartialEq)]
    enum Json {
        Null,
        Bool(bool),
        Integer(u64),
        String(String),
        Array(Vec<Json>),
        Object(HashMap<String, Json>),
    }

    /// This parser is a bit hacky due to it being written as `() -> impl Parser` instead of
    /// directly as `(&str) -> ParseResult`. I realized mid-way and was too lazy to rewrite it.
    /// But hey, it works.
    /// It's not fully JSON compliant (e.g. it supports trailing commas and does not parse floating
    /// point numbers), but it's good enough to test the parser combinators.
    #[allow(clippy::type_complexity)]
    fn json_parser() -> Box<dyn FnMut(&str) -> ParseResult<&str, Json>> {
        use super::{
            delimited_by, followed_by, map, opt, or, preceded_by, repeated, sequence,
            string_parser, take_while, try_map,
        };

        fn skip_whitespace<P, T>(parser: P) -> impl FnMut(&str) -> ParseResult<&str, T>
        where
            P: Parser<T>,
        {
            preceded_by(take_while(|c| c.is_whitespace()), parser)
        }
        let null = map(string_parser("null"), |_| Json::Null);
        let bool = or(
            map(string_parser("true"), |_| Json::Bool(true)),
            map(string_parser("false"), |_| Json::Bool(false)),
        );
        let integer = try_map(take_while(|c| c.is_ascii_digit()), |digits| {
            Ok(Json::Integer(
                digits.parse::<u64>().map_err(|e| format!("{e:?}"))?,
            ))
        });

        fn string() -> impl FnMut(&str) -> ParseResult<&str, Json> {
            map(delimited_by('"', '"', take_while(|c| c != '"')), |s| {
                Json::String(s)
            })
        }

        // Needed to break the recursion
        fn lazy<F: Fn() -> P, P: Parser<T>, T>(f: F) -> impl Fn(&str) -> ParseResult<&str, T> {
            move |input: &str| f().parse(input)
        }

        let array_item = skip_whitespace(followed_by(
            lazy(json_parser),
            skip_whitespace(opt(char_parser(','))),
        ));
        let array = map(delimited_by('[', ']', repeated(array_item)), |values| {
            Json::Array(values)
        });

        let key = skip_whitespace(followed_by(string(), skip_whitespace(char_parser(':'))));
        let object_item = followed_by(
            sequence(key, lazy(json_parser)),
            skip_whitespace(opt(char_parser(','))),
        );
        let object = map(delimited_by('{', '}', repeated(object_item)), |values| {
            Json::Object(
                values
                    .into_iter()
                    .map(|(key, value)| {
                        let key = match key {
                            Json::String(string) => string,
                            _ => unreachable!(),
                        };
                        (key, value)
                    })
                    .collect(),
            )
        });

        Box::new(skip_whitespace(or(
            or(or(or(or(null, bool), integer), string()), array),
            object,
        )))
    }

    #[track_caller]
    fn check<P, T>(mut parser: P, input: &str, expected_rest: &str, expected: T)
    where
        P: Parser<T>,
        T: Eq + Debug,
    {
        let (rest, result) = parser.parse(input).expect("Parser failed");
        assert_eq!(rest, expected_rest);
        assert_eq!(result, expected);
    }

    #[track_caller]
    fn check_fail<P, T>(mut parser: P, input: &str)
    where
        P: Parser<T>,
        T: Debug,
    {
        parser
            .parse(input)
            .expect_err("Parser did not fail even though failure was expected");
    }
}
