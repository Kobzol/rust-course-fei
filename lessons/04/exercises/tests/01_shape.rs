//! Run this file with `cargo test --test 01_shape`.

//! TODO: Create a trait `Shape` with methods for calculating the area and perimeter of a geometrical
//! object. Then create two simple geometrical objects (`Rectangle` and `Circle`) and implement
//! the `Shape` trait for both of them.


/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::{Circle, Rectangle, Shape};
    use std::f64::consts::PI;

    #[test]
    fn rectangle1() {
        let rectangle = Rectangle::new(5.0, 3.0);
        assert_almost_eq(rectangle.area(), 15.0);
        assert_almost_eq(rectangle.perimeter(), 16.0);
    }

    #[test]
    fn rectangle2() {
        let rectangle = Rectangle::new(0.3, 1982.3);
        assert_almost_eq(rectangle.area(), 594.69);
        assert_almost_eq(rectangle.perimeter(), 3965.2);
    }

    #[test]
    fn rectangle3() {
        let rectangle = Rectangle::new(0.0, 1.0);
        assert_almost_eq(rectangle.area(), 0.0);
        assert_almost_eq(rectangle.perimeter(), 2.0);
    }

    #[test]
    fn circle1() {
        let rectangle = Circle::new(5.0);
        assert_almost_eq(rectangle.area(), 25.0 * PI);
        assert_almost_eq(rectangle.perimeter(), 10.0 * PI);
    }

    #[test]
    fn circle2() {
        let rectangle = Circle::new(122038.12);
        assert_almost_eq(rectangle.area(), 46788690454.10);
        assert_almost_eq(rectangle.perimeter(), 766788.122);
    }

    #[test]
    fn circle3() {
        let rectangle = Circle::new(0.0);
        assert_almost_eq(rectangle.area(), 0.0);
        assert_almost_eq(rectangle.perimeter(), 0.0);
    }

    #[test]
    fn test_implements_trait() {
        fn take_shape<T: Shape>(_: T) {}

        take_shape(Circle::new(1.0));
        take_shape(Rectangle::new(1.0, 1.0));
    }

    #[track_caller]
    fn assert_almost_eq(value: f64, expected: f64) {
        assert!(
            (value - expected).abs() < 0.01,
            "{value} does not equal {expected}"
        );
    }
}
