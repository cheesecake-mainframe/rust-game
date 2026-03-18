#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {
            width: if width < 0.0 { 0.0 } else { width },
            height: if height < 0.0 { 0.0 } else { height },
        }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * self.width + 2.0 * self.height
    }

    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }

    fn scale(&mut self, factor: f64) {
        if factor >= 0.0 {
            self.width *= factor;
            self.height *= factor;
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

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
