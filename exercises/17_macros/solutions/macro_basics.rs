// Solution: Macro Basics
// =======================

// FIX 1: Changed `$x:value` to `$x:expr` — "value" is not a valid designator.
macro_rules! double {
    ($x:expr) => {
        $x * 2
    };
}

// FIX 2: Changed `$n:identifier` to `$n:ident` and fixed body to use `$n`.
macro_rules! create_function {
    ($n:ident) => {
        fn $n() {
            println!("Called function: {}", stringify!($n));
        }
    };
}

// FIX 3: Changed `$t:type` to `$t:ty` — the correct designator for types.
macro_rules! create_default {
    ($t:ty, $val:expr) => {
        impl Default for $t {
            fn default() -> Self {
                $val
            }
        }
    };
}

// FIX 4: Added commas in the repetition pattern `$( $element:expr ),*`
// and added `*` after the push repetition block.
macro_rules! make_list {
    ( $( $element:expr ),* ) => {
        {
            let mut list = Vec::new();
            $( list.push($element); )*
            list
        }
    };
}

// FIX 5: Added semicolons between macro arms (each arm must end with `;`).
macro_rules! calculate {
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    (mul $a:expr, $b:expr) => {
        $a * $b
    };
    (neg $a:expr) => {
        -$a
    };
}

// FIX 6: Added commas in the repetition pattern: `$( ... ),*`
macro_rules! hash_map {
    ( $( $key:expr => $value:expr ),* ) => {
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
