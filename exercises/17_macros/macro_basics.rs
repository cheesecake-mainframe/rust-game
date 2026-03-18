// Exercise: Macro Basics (FixCompilerError)
// ==========================================
//
// Rust macros use `macro_rules!` to define pattern-based code generation.
// Macros use "designators" to capture different kinds of syntax:
//   - `$x:expr`  — matches an expression (like `1 + 2`, `foo()`, `"hello"`)
//   - `$x:ident` — matches an identifier (like `foo`, `my_var`)
//   - `$x:ty`    — matches a type (like `i32`, `String`, `Vec<u8>`)
//   - `$x:literal` — matches a literal value (like `42`, `"hi"`, `true`)
//   - `$x:stmt`  — matches a statement
//   - `$x:block` — matches a block `{ ... }`
//
// Repetition patterns use `$( ... ),*` for zero-or-more comma-separated items
// and `$( ... ),+` for one-or-more.
//
// This file has several broken macro definitions. Fix them so they compile!

// FIX 1: The designator is wrong. This macro should take an expression,
// not a "value" (which isn't a valid designator).
macro_rules! double {
    ($x:value) => {
        $x * 2
    };
}

// FIX 2: The macro body references `$name` but the parameter is called `$n`.
// Also, the designator should be `ident` not `identifier`.
macro_rules! create_function {
    ($n:identifier) => {
        fn $name() {
            println!("Called function: {}", stringify!($name));
        }
    };
}

// FIX 3: This macro takes a type and a default value, but the repetition
// syntax is wrong. The `*` should come after the closing paren, and
// the designator for the type is misspelled.
macro_rules! create_default {
    ($t:type, $val:expr) => {
        impl Default for $t {
            fn default() -> Self {
                $val
            }
        }
    };
}

// FIX 4: This macro should accept zero or more expressions separated by
// commas, but the repetition pattern syntax is broken.
// Hint: The pattern should be $( $element:expr ),* and the body
// should repeat the push calls with $( ... )*
macro_rules! make_list {
    ( $( $element:expr )* ) => {
        {
            let mut list = Vec::new();
            $( list.push($element); )
            list
        }
    };
}

// FIX 5: This macro has multiple arms but the pattern matching syntax
// is wrong. Each arm should end with a semicolon, and the patterns
// need to be properly separated.
macro_rules! calculate {
    (add $a:expr, $b:expr) => {
        $a + $b
    }
    (mul $a:expr, $b:expr) => {
        $a * $b
    }
    (neg $a:expr) => {
        -$a
    }
}

// FIX 6: This macro should create a HashMap from key => value pairs.
// The repetition pattern is missing commas between captured items,
// and the body doesn't repeat correctly.
macro_rules! hash_map {
    ( $( $key:expr => $value:expr )* ) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert($key, $value); )*
            map
        }
    };
}

fn main() {
    // Test double
    let x = double!(5);
    println!("double(5) = {}", x);
    assert_eq!(x, 10);

    // Test create_function
    create_function!(hello);
    hello();

    // Test make_list
    let list = make_list![1, 2, 3, 4, 5];
    println!("list = {:?}", list);
    assert_eq!(list, vec![1, 2, 3, 4, 5]);

    // Test calculate
    let sum = calculate!(add 3, 4);
    let product = calculate!(mul 3, 4);
    let negated = calculate!(neg 5);
    println!("3 + 4 = {}", sum);
    println!("3 * 4 = {}", product);
    println!("-5 = {}", negated);
    assert_eq!(sum, 7);
    assert_eq!(product, 12);
    assert_eq!(negated, -5);

    // Test hash_map
    let map = hash_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3
    };
    println!("map = {:?}", map);
    assert_eq!(map["one"], 1);
    assert_eq!(map["two"], 2);
    assert_eq!(map["three"], 3);

    println!("All macro tests passed!");
}

#[cfg(test)]
mod tests {
    // Note: macros are available in child modules by default

    #[test]
    fn test_double() {
        assert_eq!(double!(0), 0);
        assert_eq!(double!(7), 14);
        assert_eq!(double!(-3), -6);
    }

    #[test]
    fn test_make_list() {
        let empty: Vec<i32> = make_list![];
        assert!(empty.is_empty());

        let single = make_list![42];
        assert_eq!(single, vec![42]);

        let multi = make_list![1, 2, 3];
        assert_eq!(multi, vec![1, 2, 3]);
    }

    #[test]
    fn test_calculate() {
        assert_eq!(calculate!(add 10, 20), 30);
        assert_eq!(calculate!(mul 5, 6), 30);
        assert_eq!(calculate!(neg 42), -42);
    }

    #[test]
    fn test_hash_map() {
        let m = hash_map! { "a" => 1 };
        assert_eq!(m.len(), 1);
        assert_eq!(m["a"], 1);
    }
}
