use std::collections::HashMap;

mod shapes {
    pub struct Circle {
        pub radius: f64,
    }

    pub struct Square {
        pub side: f64,
    }

    impl Circle {
        pub fn new(radius: f64) -> Circle {
            Circle { radius }
        }

        pub fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    impl Square {
        pub fn new(side: f64) -> Square {
            Square { side }
        }

        pub fn area(&self) -> f64 {
            self.side * self.side
        }
    }
}

mod colors {
    pub enum Color {
        Red,
        Green,
        Blue,
        Custom(u8, u8, u8),
    }

    impl Color {
        pub fn to_hex(&self) -> String {
            match self {
                Color::Red => "#FF0000".to_string(),
                Color::Green => "#00FF00".to_string(),
                Color::Blue => "#0000FF".to_string(),
                Color::Custom(r, g, b) => format!("#{:02X}{:02X}{:02X}", r, g, b),
            }
        }
    }
}

use shapes::{Circle, Square};
use colors::Color;

fn main() {
    let c = Circle::new(5.0);
    let s = Square::new(4.0);

    println!("Circle area: {:.2}", c.area());
    println!("Square area: {:.2}", s.area());

    let red = Color::Red;
    let custom = Color::Custom(128, 64, 32);
    println!("Red: {}", red.to_hex());
    println!("Custom: {}", custom.to_hex());

    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    println!("Scores: {:?}", scores);
}
