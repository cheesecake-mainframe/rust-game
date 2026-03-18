fn make_excited(mut s: String) -> String {
    s.push('!');
    s
}

fn push_and_return(mut v: Vec<i32>, item: i32) -> Vec<i32> {
    v.push(item);
    v
}

fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_excited() {
        let s = String::from("hello");
        let result = make_excited(s);
        assert_eq!(result, "hello!");
    }

    #[test]
    fn test_push_and_return() {
        let v = vec![1, 2, 3];
        let result = push_and_return(v, 4);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_create_greeting() {
        let g = create_greeting("Ferris");
        assert_eq!(g, "Hello, Ferris!");
    }

    #[test]
    fn test_create_greeting_world() {
        let g = create_greeting("world");
        assert_eq!(g, "Hello, world!");
    }
}
