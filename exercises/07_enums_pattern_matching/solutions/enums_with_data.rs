use std::f64::consts::PI;

enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn describe(&self) -> String {
        match self {
            Shape::Circle { radius } => format!("Circle with radius {}", radius),
            Shape::Rectangle { width, height } => format!("Rectangle {}x{}", width, height),
            Shape::Triangle { base, height } => {
                format!("Triangle with base {} and height {}", base, height)
            }
        }
    }

    fn is_larger_than(&self, other: &Shape) -> bool {
        self.area() > other.area()
    }
}

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
