// Exercise: Enums with Data
// Type: implement_from_scratch
// Difficulty: intermediate
//
// Rust enums are much more powerful than enums in most languages.
// Each variant can hold different types and amounts of data —
// like a tagged union or algebraic data type.
//
// Combined with `match`, you can destructure enum variants and
// access their inner data.
//
// Implement the Shape enum and its area() method to make the tests pass.

use std::f64::consts::PI;

// TODO: Define a Shape enum with these variants:
//   - Circle { radius: f64 }
//   - Rectangle { width: f64, height: f64 }
//   - Triangle { base: f64, height: f64 }

// TODO: Implement these methods for Shape:
//
// - area(&self) -> f64
//       Use match to calculate the area based on the variant:
//       Circle: PI * radius * radius
//       Rectangle: width * height
//       Triangle: 0.5 * base * height
//
// - describe(&self) -> String
//       Return a human-readable description, e.g.:
//       "Circle with radius 5"
//       "Rectangle 10x5"
//       "Triangle with base 6 and height 4"
//
// - is_larger_than(&self, other: &Shape) -> bool
//       Return true if self has a larger area than other.

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let c = Shape::Circle { radius: 5.0 };
        let expected = PI * 25.0;
        assert!((c.area() - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn test_rectangle_area() {
        let r = Shape::Rectangle { width: 10.0, height: 5.0 };
        assert!((r.area() - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_triangle_area() {
        let t = Shape::Triangle { base: 6.0, height: 4.0 };
        assert!((t.area() - 12.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_describe_circle() {
        let c = Shape::Circle { radius: 5.0 };
        assert_eq!(c.describe(), "Circle with radius 5");
    }

    #[test]
    fn test_describe_rectangle() {
        let r = Shape::Rectangle { width: 10.0, height: 5.0 };
        assert_eq!(r.describe(), "Rectangle 10x5");
    }

    #[test]
    fn test_describe_triangle() {
        let t = Shape::Triangle { base: 6.0, height: 4.0 };
        assert_eq!(t.describe(), "Triangle with base 6 and height 4");
    }

    #[test]
    fn test_is_larger_than() {
        let big = Shape::Rectangle { width: 100.0, height: 100.0 };
        let small = Shape::Circle { radius: 1.0 };
        assert!(big.is_larger_than(&small));
        assert!(!small.is_larger_than(&big));
    }

    #[test]
    fn test_equal_areas() {
        let s = Shape::Rectangle { width: 5.0, height: 5.0 };
        let same = Shape::Rectangle { width: 5.0, height: 5.0 };
        assert!(!s.is_larger_than(&same));
    }
}
