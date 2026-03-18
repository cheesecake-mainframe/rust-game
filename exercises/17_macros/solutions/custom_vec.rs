// Solution: Custom Vec Macro
// ===========================

macro_rules! my_vec {
    // Arm 1: Empty vec
    () => {
        Vec::new()
    };

    // Arm 2: Repeated value — `my_vec![val; count]`
    ($elem:expr; $count:expr) => {
        vec![$elem; $count]
    };

    // Arm 3: List of elements with optional trailing comma
    ($( $elem:expr ),+ $(,)?) => {
        {
            let mut v = Vec::new();
            $( v.push($elem); )+
            v
        }
    };
}

fn main() {
    let empty: Vec<i32> = my_vec![];
    println!("empty: {:?}", empty);

    let numbers = my_vec![1, 2, 3, 4, 5];
    println!("numbers: {:?}", numbers);

    let zeros = my_vec![0; 10];
    println!("zeros: {:?}", zeros);

    let strings = my_vec!["hello", "world"];
    println!("strings: {:?}", strings);

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
