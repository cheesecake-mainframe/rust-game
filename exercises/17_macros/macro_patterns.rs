// Exercise: Macro Patterns (ImplementFromScratch)
// ================================================
//
// Macros can use different designators to capture various kinds of Rust syntax.
// This exercise explores several designator types and repetition patterns.
//
// Implement each macro according to its specification and tests.
//
// Designator reference:
//   $x:expr    — an expression (1+2, foo(), "hello")
//   $x:ty      — a type (i32, String, Vec<u8>)
//   $x:ident   — an identifier (my_var, foo, MyStruct)
//   $x:literal — a literal (42, "hi", true, 3.14)
//   $x:stmt    — a statement (let x = 1)
//   $x:block   — a block { ... }
//   $x:pat     — a pattern (Some(x), (a, b), _)
//   $x:tt      — a single token tree

// TODO 1: Implement `say_hello!`
// This macro takes an identifier (a name) and prints "Hello, <name>!".
// Usage: say_hello!(world) => prints "Hello, world!"
// Hint: Use `stringify!` to convert an ident to a string.

// TODO 2: Implement `create_struct!`
// This macro takes a struct name ($name:ident) and zero or more field
// definitions ($field_name:ident : $field_type:ty), and creates a struct.
//
// Usage:
//   create_struct!(Point, x: f64, y: f64);
// Generates:
//   #[derive(Debug, PartialEq)]
//   struct Point { x: f64, y: f64 }

// TODO 3: Implement `math!`
// This macro implements a small DSL for math operations using custom syntax.
// It should support these patterns:
//   math!(square $x:expr)     => $x * $x
//   math!(cube $x:expr)       => $x * $x * $x
//   math!(avg $a:expr, $b:expr) => ($a + $b) / 2

// TODO 4: Implement `count_exprs!`
// This macro counts the number of expressions passed to it.
// Usage:
//   count_exprs!()          => 0
//   count_exprs!(a)         => 1
//   count_exprs!(a, b, c)   => 3
//
// Hint: One approach uses a nested macro trick: replace each expression
// with a unit `()` in an array and take its length.
// `{ let arr: &[()] = &[$( { let _ = $e; } ),*]; arr.len() }`
// Or simpler: `0 $( + { let _ = $e; 1 } )*`

// TODO 5: Implement `implement_ops!`
// This macro takes a type name and creates a `describe()` method on it
// that returns a String describing the type.
//
// Usage:
//   implement_ops!(MyType);
// Generates:
//   impl MyType {
//       fn describe(&self) -> String {
//           String::from("I am a MyType")
//       }
//   }
//
// Hint: Use `concat!("I am a ", stringify!($name))` to build the string.

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
