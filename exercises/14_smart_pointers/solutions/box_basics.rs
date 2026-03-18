// ========================================
// Solution: Box Basics
// ========================================

// Fix: Wrap the recursive field in Box<T> so the compiler knows the size.
// Without Box, List would contain List which contains List... infinite size.
// Box<List> is just a pointer (fixed size) pointing to a heap-allocated List.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    // Fix: Wrap `self` in Box::new() when constructing Cons.
    fn push(self, value: i32) -> List {
        List::Cons(value, Box::new(self))
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = self;
        loop {
            match current {
                // Fix: Pattern now matches Box<List> in the second field.
                List::Cons(val, next) => {
                    result.push(*val);
                    current = next;
                }
                List::Nil => break,
            }
        }
        result.reverse();
        result
    }

    fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self;
        loop {
            match current {
                List::Cons(_, next) => {
                    count += 1;
                    current = next;
                }
                List::Nil => break,
            }
        }
        count
    }
}

// Fix: Wrap recursive children in Box<T>.
enum BinaryTree {
    Leaf(i32),
    Node(i32, Box<BinaryTree>, Box<BinaryTree>),
}

impl BinaryTree {
    fn leaf(value: i32) -> BinaryTree {
        BinaryTree::Leaf(value)
    }

    // Fix: Wrap left and right in Box::new().
    fn node(value: i32, left: BinaryTree, right: BinaryTree) -> BinaryTree {
        BinaryTree::Node(value, Box::new(left), Box::new(right))
    }

    fn sum(&self) -> i32 {
        match self {
            BinaryTree::Leaf(v) => *v,
            // Box<T> implements Deref, so left.sum() works through auto-deref.
            BinaryTree::Node(v, left, right) => v + left.sum() + right.sum(),
        }
    }
}

trait Describable {
    fn describe(&self) -> String;
}

struct Cat {
    name: String,
}

struct Dog {
    name: String,
}

impl Describable for Cat {
    fn describe(&self) -> String {
        format!("{} the cat", self.name)
    }
}

impl Describable for Dog {
    fn describe(&self) -> String {
        format!("{} the dog", self.name)
    }
}

// Fix: Return Box<dyn Describable> -- a heap-allocated trait object.
// This allows returning different concrete types (Cat or Dog) from the same function.
fn make_animal(is_cat: bool) -> Box<dyn Describable> {
    if is_cat {
        Box::new(Cat { name: String::from("Whiskers") })
    } else {
        Box::new(Dog { name: String::from("Rex") })
    }
}

fn main() {
    // Test linked list
    let list = List::new()
        .push(1)
        .push(2)
        .push(3);
    assert_eq!(list.to_vec(), vec![1, 2, 3]);
    assert_eq!(list.len(), 3);

    let empty = List::new();
    assert_eq!(empty.to_vec(), Vec::<i32>::new());
    assert_eq!(empty.len(), 0);

    // Test binary tree
    let tree = BinaryTree::node(
        5,
        BinaryTree::node(
            3,
            BinaryTree::leaf(1),
            BinaryTree::leaf(4),
        ),
        BinaryTree::leaf(8),
    );
    assert_eq!(tree.sum(), 21);

    // Test trait objects
    let cat = make_animal(true);
    let dog = make_animal(false);
    assert_eq!(cat.describe(), "Whiskers the cat");
    assert_eq!(dog.describe(), "Rex the dog");

    println!("All Box basics tests passed!");
}
