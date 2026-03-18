// ========================================
// Exercise: Box Basics (FixCompilerError)
// ========================================
// Difficulty: Intermediate
// Module: 14 - Smart Pointers
//
// CONCEPT:
// Box<T> is the simplest smart pointer in Rust. It allocates data on the heap
// instead of the stack. Box is most commonly used when:
//   - You have a type whose size can't be known at compile time (recursive types)
//   - You want to transfer ownership without copying large data
//   - You want to own a value and only care that it implements a trait (trait objects)
//
// Recursive types are the classic use case: an enum variant that contains itself
// has infinite size. Wrapping the recursive part in Box<T> gives it a known size
// (a pointer is always a fixed size).
//
// YOUR TASK:
// Fix each section so the code compiles. The main issues are:
// - Recursive types need Box to have a known size
// - Some operations require heap allocation
// - Trait objects need Box<dyn Trait>
// ========================================

// FIX: This enum is recursive -- `List` contains `List` directly, which gives
// it infinite size. Wrap the recursive field in Box<T>.
enum List {
    Cons(i32, List),
    Nil,
}

impl List {
    // FIX: Update this function to match the fixed enum definition.
    fn new() -> List {
        List::Nil
    }

    // FIX: Update this function to match the fixed enum definition.
    fn push(self, value: i32) -> List {
        List::Cons(value, self)
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = self;
        loop {
            match current {
                // FIX: Update the pattern to match the fixed enum.
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
                // FIX: Update the pattern to match the fixed enum.
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

// FIX: This recursive tree type also has infinite size.
// Wrap the children in Box<T>.
enum BinaryTree {
    Leaf(i32),
    Node(i32, BinaryTree, BinaryTree),
}

impl BinaryTree {
    // FIX: Update to match the fixed enum.
    fn leaf(value: i32) -> BinaryTree {
        BinaryTree::Leaf(value)
    }

    // FIX: Update to match the fixed enum.
    fn node(value: i32, left: BinaryTree, right: BinaryTree) -> BinaryTree {
        BinaryTree::Node(value, left, right)
    }

    fn sum(&self) -> i32 {
        match self {
            BinaryTree::Leaf(v) => *v,
            // FIX: Update the pattern to match the fixed enum.
            BinaryTree::Node(v, left, right) => v + left.sum() + right.sum(),
        }
    }
}

// FIX: This function should return a trait object. The return type needs Box<dyn ...>.
// A Box<dyn Trait> lets you return different concrete types behind a single interface.
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

// FIX: The return type `Describable` is a trait, not a sized type.
// Use Box<dyn Describable> to return a trait object.
fn make_animal(is_cat: bool) -> Describable {
    if is_cat {
        Cat { name: String::from("Whiskers") }
    } else {
        Dog { name: String::from("Rex") }
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
    //       5
    //      / \
    //     3   8
    //    / \
    //   1   4
    let tree = BinaryTree::node(
        5,
        BinaryTree::node(
            3,
            BinaryTree::leaf(1),
            BinaryTree::leaf(4),
        ),
        BinaryTree::leaf(8),
    );
    assert_eq!(tree.sum(), 21); // 5 + 3 + 1 + 4 + 8

    // Test trait objects
    let cat = make_animal(true);
    let dog = make_animal(false);
    assert_eq!(cat.describe(), "Whiskers the cat");
    assert_eq!(dog.describe(), "Rex the dog");

    println!("All Box basics tests passed!");
}
