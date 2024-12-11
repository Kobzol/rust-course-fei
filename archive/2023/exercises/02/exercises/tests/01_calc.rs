//! Run this file with `cargo test --test 01_calc`.
// Implement an enum that is passed to the `perform_calculation` function
// (see tests), and then implement the function.
// Hint: max(..) and min(..) methods of `i32` might come in handy.

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::{perform_calculation, Op};

    #[test]
    fn calc_add() {
        assert_eq!(perform_calculation(0, Op::Add(1)), 1);
        assert_eq!(perform_calculation(3, Op::Add(10)), 13);
    }

    #[test]
    fn calc_sub() {
        assert_eq!(perform_calculation(0, Op::Sub(10)), -10);
        assert_eq!(perform_calculation(3000, Op::Sub(-5)), 3005);
    }

    /// Clamp makes sure that a value is between a minimum and maximum value
    /// (inclusive).
    /// clamp(1, 0, 8)     = 1
    /// clamp(-5, 0, 8)    = 0
    /// clamp(-5, -15, 9)  = -5
    /// clamp(50, 0, 8)    = 8
    /// clamp(50, 0, 80)   = 50
    #[test]
    fn calc_clamp() {
        assert_eq!(perform_calculation(0, Op::Clamp { low: 0, high: 0 }), 0);
        assert_eq!(perform_calculation(5, Op::Clamp { low: 0, high: 0 }), 0);
        assert_eq!(perform_calculation(3, Op::Clamp { low: 2, high: 8 }), 3);
        assert_eq!(perform_calculation(-5, Op::Clamp { low: 0, high: 10 }), 0);
        assert_eq!(perform_calculation(50, Op::Clamp { low: 3, high: 10 }), 10);
        assert_eq!(perform_calculation(50, Op::Clamp { low: 3, high: 100 }), 50);
    }
}
