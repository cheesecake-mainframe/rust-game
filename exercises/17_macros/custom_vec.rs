// Exercise: Custom Vec Macro (ImplementFromScratch)
// ==================================================
//
// The standard library provides the `vec!` macro for creating vectors easily:
//   let v = vec![1, 2, 3];
//
// Your task is to implement `my_vec!`, a custom macro that works just like
// `vec!`. It should support three forms:
//
// 1. Empty:          `my_vec![]`          -> creates an empty Vec
// 2. List of items:  `my_vec![1, 2, 3]`  -> creates Vec with those elements
// 3. Repeated value: `my_vec![0; 5]`     -> creates Vec with 5 zeros
//
// HINTS:
// - Use `macro_rules!` to define the macro
// - You'll need multiple arms (patterns) in the macro
// - For the list form, use repetition: `$( $elem:expr ),*`
// - For the repeat form, match `$elem:expr ; $count:expr`
// - Don't forget to handle trailing commas: `$( $elem:expr ),* $(,)?`
// - The macro should create a new Vec, push elements, and return it
//   (or use a more elegant approach)

// TODO: Implement the `my_vec!` macro here.
// It should support these three forms:
//
// 1. my_vec![]           -> empty Vec<T>
// 2. my_vec![a, b, c]    -> Vec containing a, b, c
// 3. my_vec![val; count] -> Vec with `count` copies of `val`

fn main() {
    // These should all work once you implement my_vec!
    let empty: Vec<i32> = my_vec![];
    println!("empty: {:?}", empty);

    let numbers = my_vec![1, 2, 3, 4, 5];
    println!("numbers: {:?}", numbers);

    let zeros = my_vec![0; 10];
    println!("zeros: {:?}", zeros);

    let strings = my_vec!["hello", "world"];
    println!("strings: {:?}", strings);

    // With trailing comma
    let trailing = my_vec![1, 2, 3,];
    println!("trailing: {:?}", trailing);

    println!("All my_vec! invocations worked!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_vec() {
        let v: Vec<i32> = my_vec![];
        assert!(v.is_empty());
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_single_element() {
        let v = my_vec![42];
        assert_eq!(v, vec![42]);
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn test_multiple_elements() {
        let v = my_vec![1, 2, 3, 4, 5];
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_trailing_comma() {
        let v = my_vec![10, 20, 30,];
        assert_eq!(v, vec![10, 20, 30]);
    }

    #[test]
    fn test_repeated_value() {
        let v = my_vec![7; 4];
        assert_eq!(v, vec![7, 7, 7, 7]);
        assert_eq!(v.len(), 4);
    }

    #[test]
    fn test_repeated_zero() {
        let v = my_vec![0; 100];
        assert_eq!(v.len(), 100);
        assert!(v.iter().all(|&x| x == 0));
    }

    #[test]
    fn test_string_elements() {
        let v = my_vec!["a", "b", "c"];
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_expression_elements() {
        let v = my_vec![1 + 1, 2 * 3, 10 / 2];
        assert_eq!(v, vec![2, 6, 5]);
    }

    #[test]
    fn test_repeated_string() {
        let v = my_vec!["hello"; 3];
        assert_eq!(v, vec!["hello", "hello", "hello"]);
    }
}
