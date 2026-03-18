// Solution: Macro Patterns
// =========================

// Macro 1: say_hello! — prints "Hello, <name>!"
macro_rules! say_hello {
    ($name:ident) => {
        println!("Hello, {}!", stringify!($name));
    };
}

// Macro 2: create_struct! — generates a struct with derive(Debug, PartialEq)
macro_rules! create_struct {
    ($name:ident, $( $field_name:ident : $field_type:ty ),* $(,)?) => {
        #[derive(Debug, PartialEq)]
        struct $name {
            $( $field_name: $field_type, )*
        }
    };
}

// Macro 3: math! — a small DSL for math operations
macro_rules! math {
    (square $x:expr) => {
        $x * $x
    };
    (cube $x:expr) => {
        $x * $x * $x
    };
    (avg $a:expr, $b:expr) => {
        ($a + $b) / 2
    };
}

// Macro 4: count_exprs! — counts the number of expressions passed
macro_rules! count_exprs {
    () => { 0 };
    ($( $e:expr ),+ $(,)?) => {
        0 $( + { let _ = &$e; 1 } )+
    };
}

// Macro 5: implement_ops! — creates a describe() method on a type
macro_rules! implement_ops {
    ($name:ident) => {
        impl $name {
            fn describe(&self) -> String {
                String::from(concat!("I am a ", stringify!($name)))
            }
        }
    };
}

fn main() {
    // Test say_hello
    say_hello!(world);
    say_hello!(Rust);

    // Test create_struct
    create_struct!(Color, r: u8, g: u8, b: u8);
    let red = Color { r: 255, g: 0, b: 0 };
    println!("{:?}", red);

    // Test math
    println!("square(5) = {}", math!(square 5));
    println!("cube(3) = {}", math!(cube 3));
    println!("avg(10, 20) = {}", math!(avg 10, 20));

    // Test count_exprs
    println!("count_exprs!() = {}", count_exprs!());
    println!("count_exprs!(1) = {}", count_exprs!(1));
    println!("count_exprs!(1, 2, 3) = {}", count_exprs!(1, 2, 3));

    // Test implement_ops
    struct Robot;
    implement_ops!(Robot);
    let r = Robot;
    println!("{}", r.describe());

    println!("All macro pattern tests passed!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_struct() {
        create_struct!(Point2D, x: f64, y: f64);
        let p = Point2D { x: 1.0, y: 2.0 };
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
    }

    #[test]
    fn test_create_struct_single_field() {
        create_struct!(Wrapper, value: i32);
        let w = Wrapper { value: 42 };
        assert_eq!(w.value, 42);
    }

    #[test]
    fn test_math_square() {
        assert_eq!(math!(square 4), 16);
        assert_eq!(math!(square 0), 0);
        assert_eq!(math!(square 10), 100);
    }

    #[test]
    fn test_math_cube() {
        assert_eq!(math!(cube 2), 8);
        assert_eq!(math!(cube 3), 27);
    }

    #[test]
    fn test_math_avg() {
        assert_eq!(math!(avg 10, 20), 15);
        assert_eq!(math!(avg 0, 100), 50);
    }

    #[test]
    fn test_count_exprs_zero() {
        assert_eq!(count_exprs!(), 0);
    }

    #[test]
    fn test_count_exprs_one() {
        assert_eq!(count_exprs!("hello"), 1);
    }

    #[test]
    fn test_count_exprs_many() {
        assert_eq!(count_exprs!(1, 2, 3, 4, 5), 5);
    }

    #[test]
    fn test_implement_ops() {
        struct Widget;
        implement_ops!(Widget);
        let w = Widget;
        assert_eq!(w.describe(), "I am a Widget");
    }
}
