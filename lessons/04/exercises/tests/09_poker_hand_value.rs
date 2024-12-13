//! Run this file with `cargo test --test 09_poker_hand_value`.

//! TODO: let's play some poker :)
//!
//! Below you can find a representation of cards and a "poker hand" with exactly 5 cards drawn.
//! You have two tasks:
//! - Implement a method called `value` on `Hand`, which returns the highest possible `HandValue`
//!   of the hand. The rules of the different values are explained in the `HandValue` enum.
//!   Make sure to return the highest value - if a hand contains `HandValue::FiveOfAKind`, it
//!   definitely also satisfies `HandValue::FourOfAKind`, but you should return `FiveOfAKind`.
//! - Implement `PartialOrd` for `Hand`, so that we can compare and sort hands by their value.
//!   Hands `a` and `b` should be compared in the following way:
//!   - If `a` has lower value than `b`, then `a < b`
//!   - If `a` has higher value than `b`, then `a > b`
//!   - If they have the same value, then compare cards of the hands lexicographically.
//!     Go through individual cards of each hand starting from the left. When you encounter the
//!     first place where `a` and `b` have a different card, compare the hands based on this card.
//!     For example, "25456" and "25465" have the same value (`OnePair`).
//!     The first place where they differ is the fourth card ("5" vs "6"). Therefore, we return
//!     the result of comparing these two cards.
//!
//! Note: card counts are not limited in any way here (which normally happens with a 52 card deck,
//! where there are exactly 4 pieces of each card). Therefore, e.g. 'AAAAA' is also a valid hand.
//!
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Write};

// Card values are sorted in ascending order (Numeric < Jack < Queen < King < Ace).
// This is achieved by `#[derive(PartialOrd)]`, which sorts the variants in source code
// order.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd)]
enum Card {
    Numeric(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Numeric(v) => v.fmt(f),
            Card::Jack => f.write_char('J'),
            Card::Queen => f.write_char('Q'),
            Card::King => f.write_char('K'),
            Card::Ace => f.write_char('A'),
        }
    }
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
}

impl Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in &self.cards {
            c.fmt(f)?;
        }
        Ok(())
    }
}

// Hand values are sorted from lowest (HighCard) to highest (FiveOfAKind).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandValue {
    /// All five cards are different
    /// e.g. "23456"
    HighCard,
    /// There is exactly one pair of equal cards
    /// e.g. "22345"
    OnePair,
    /// There are two pairs of exact cards
    /// e.g. "855J8"
    TwoPairs,
    /// There are three instances of the same card
    /// e.g. "8JQ88"
    ThreeOfAKind,
    /// There are three instances of one card, and a pair of a second card
    /// e.g. "AJJAA"
    FullHouse,
    /// There are four instances of one card
    /// e.g. "98888"
    FourOfAKind,
    /// All cards in the hand are the same
    /// e.g. "AAAAA"
    FiveOfAKind,
}

impl Hand {
    fn parse(input: &str) -> Result<Self, String> {
        if input.len() != 5 {
            return Err("Wrong hand length. It has to contain exactly 5 cards".to_string());
        }
        let mut cards = vec![];
        for c in input.chars() {
            let card = match c {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Jack,
                'T' => Card::Numeric(10),
                c if c.is_ascii_digit() => {
                    let digit = (c as u32 - '0' as u32) as u8;
                    if digit < 2 {
                        return Err(format!("Invalid card value {digit}"));
                    }
                    Card::Numeric(digit)
                }
                _ => return Err(format!("Invalid card {c}")),
            };
            cards.push(card);
        }

        Ok(Self {
            cards: cards.try_into().unwrap(),
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    // TODO: implement this method
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

impl Hand {
    // TODO: implement this method
    fn value(&self) -> HandValue {
        todo!()
    }
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::{Hand, HandValue};

    #[test]
    fn value_high_card() {
        check_value("23456", HandValue::HighCard);
    }

    #[test]
    fn value_one_pair() {
        check_value("22345", HandValue::OnePair);
        check_value("23425", HandValue::OnePair);
        check_value("JQ23J", HandValue::OnePair);
    }

    #[test]
    fn value_two_pairs() {
        check_value("22335", HandValue::TwoPairs);
        check_value("23J32", HandValue::TwoPairs);
        check_value("KKQQJ", HandValue::TwoPairs);
    }

    #[test]
    fn value_three_of_a_kind() {
        check_value("22235", HandValue::ThreeOfAKind);
        check_value("J2Q22", HandValue::ThreeOfAKind);
        check_value("QKJJJ", HandValue::ThreeOfAKind);
    }

    #[test]
    fn value_full_house() {
        check_value("22233", HandValue::FullHouse);
        check_value("23232", HandValue::FullHouse);
        check_value("QQJJJ", HandValue::FullHouse);
    }

    #[test]
    fn value_four_of_a_kind() {
        check_value("22223", HandValue::FourOfAKind);
        check_value("32222", HandValue::FourOfAKind);
        check_value("JQJJJ", HandValue::FourOfAKind);
    }

    #[test]
    fn value_five_of_a_kind() {
        check_value("22222", HandValue::FiveOfAKind);
        check_value("JJJJJ", HandValue::FiveOfAKind);
    }

    #[test]
    fn compare_exact() {
        assert_eq!(hand("22222"), hand("22222"));
        assert_eq!(hand("2345J"), hand("2345J"));
    }

    #[test]
    fn compare_different() {
        assert!(hand("Q2345") < hand("22222"));
        assert!(hand("JQJK2") < hand("23232"));
        assert!(hand("45444") > hand("23232"));
        assert!(hand("45444") > hand("44544"));
    }

    #[test]
    fn compare_same_value_different_hands() {
        assert!(hand("22222") < hand("33333"));
        assert!(hand("TTTTT") < hand("QQQQQ"));
        assert!(hand("23423") < hand("23432"));
        assert!(hand("JJQQQ") < hand("QJQQJ"));
        assert!(hand("QQQQ2") < hand("QQQQ4"));
        assert!(hand("44544") < hand("45444"));
    }

    #[test]
    fn sort_hands() {
        let mut hands = vec![
            hand("QQQQQ"),
            hand("22222"),
            hand("58685"),
            hand("Q8Q4Q"),
            hand("56884"),
            hand("QJKA5"),
            hand("QQQ2Q"),
            hand("KJKKJ"),
            hand("58658"),
        ];
        hands.sort();
        assert_eq!(
            hands,
            vec![
                hand("QJKA5"),
                hand("56884"),
                hand("58658"),
                hand("58685"),
                hand("Q8Q4Q"),
                hand("KJKKJ"),
                hand("QQQ2Q"),
                hand("22222"),
                hand("QQQQQ"),
            ]
        );
    }

    #[track_caller]
    fn check_value(input: &str, value: HandValue) {
        let hand = hand(input);
        assert_eq!(hand.value(), value);
    }

    fn hand(input: &str) -> Hand {
        Hand::parse(input).unwrap()
    }
}
