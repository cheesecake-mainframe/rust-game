// Exercise: Methods
// Type: implement_from_scratch
// Difficulty: intermediate
//
// Methods are functions attached to a struct via `impl`. The first
// parameter is always `self` (the instance):
//   - `&self`      — borrow the instance (read-only)
//   - `&mut self`  — borrow the instance (read-write)
//   - `self`       — take ownership of the instance
//
// Implement all the methods on Rectangle to make the tests pass.

#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: f64,
    height: f64,
}

// TODO: Implement these methods for Rectangle:
//
// - new(width: f64, height: f64) -> Rectangle
//       Create a new rectangle. Width and height must be non-negative;
//       clamp negative values to 0.0.
//
// - area(&self) -> f64
//       Return the area (width * height).
//
// - perimeter(&self) -> f64
//       Return the perimeter (2 * width + 2 * height).
//
// - is_square(&self) -> bool
//       Return true if width equals height.
//
// - scale(&mut self, factor: f64)
//       Multiply both width and height by factor.
//       If factor is negative, do nothing.
//
// - can_hold(&self, other: &Rectangle) -> bool
//       Return true if `other` fits entirely inside `self`.

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let r = Rectangle::new(10.0, 5.0);
        assert_eq!(r.width, 10.0);
        assert_eq!(r.height, 5.0);
    }

    #[test]
    fn test_new_clamps_negative() {
        let r = Rectangle::new(-3.0, -7.0);
        assert_eq!(r.width, 0.0);
        assert_eq!(r.height, 0.0);
    }

    #[test]
    fn test_area() {
        let r = Rectangle::new(10.0, 5.0);
        assert!((r.area() - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_perimeter() {
        let r = Rectangle::new(10.0, 5.0);
        assert!((r.perimeter() - 30.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_is_square_true() {
        let r = Rectangle::new(7.0, 7.0);
        assert!(r.is_square());
    }

    #[test]
    fn test_is_square_false() {
        let r = Rectangle::new(7.0, 5.0);
        assert!(!r.is_square());
    }

    #[test]
    fn test_scale() {
        let mut r = Rectangle::new(4.0, 3.0);
        r.scale(2.0);
        assert!((r.width - 8.0).abs() < f64::EPSILON);
        assert!((r.height - 6.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_scale_negative_does_nothing() {
        let mut r = Rectangle::new(4.0, 3.0);
        r.scale(-1.0);
        assert!((r.width - 4.0).abs() < f64::EPSILON);
        assert!((r.height - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_can_hold() {
        let big = Rectangle::new(10.0, 8.0);
        let small = Rectangle::new(5.0, 4.0);
        assert!(big.can_hold(&small));
        assert!(!small.can_hold(&big));
    }

    #[test]
    fn test_can_hold_equal() {
        let r1 = Rectangle::new(5.0, 5.0);
        let r2 = Rectangle::new(5.0, 5.0);
        assert!(r1.can_hold(&r2));
    }
}
