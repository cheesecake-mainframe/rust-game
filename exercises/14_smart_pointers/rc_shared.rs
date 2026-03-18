// ========================================
// Exercise: Rc Shared Ownership (ImplementFromScratch)
// ========================================
// Difficulty: Intermediate
// Module: 14 - Smart Pointers
//
// CONCEPT:
// Rc<T> (Reference Counted) enables shared ownership of heap data.
// Multiple parts of your program can read the same data without copying it.
// Key points:
//   - Rc::new(value) creates a new reference-counted value
//   - Rc::clone(&rc) creates another owner (increments the count, NOT deep copy)
//   - Rc::strong_count(&rc) returns the current reference count
//   - When the last Rc is dropped, the value is cleaned up
//   - Rc<T> only allows immutable access (shared references)
//   - Rc<T> is NOT thread-safe -- use Arc<T> for concurrency
//
// Use case: A tree or graph where nodes can have multiple parents/owners.
//
// YOUR TASK:
// Implement the functions below. Each one demonstrates a different aspect
// of Rc<T> and shared ownership.
// ========================================

use std::rc::Rc;

/// A simple tree node where children can be shared between multiple parents.
/// For example, a node might appear as a child of two different parent nodes.
#[derive(Debug)]
struct TreeNode {
    value: String,
    children: Vec<Rc<TreeNode>>,
}

/// Creates a new TreeNode wrapped in Rc.
/// This allows the node to be shared (have multiple owners).
fn make_node(value: &str, children: Vec<Rc<TreeNode>>) -> Rc<TreeNode> {
    todo!()
}

/// Creates a leaf node (no children) wrapped in Rc.
fn make_leaf(value: &str) -> Rc<TreeNode> {
    todo!()
}

/// Returns the reference count of the given Rc.
fn ref_count(node: &Rc<TreeNode>) -> usize {
    todo!()
}

/// Collects all values in the tree into a vector using depth-first traversal.
/// The root's value comes first, then each child subtree recursively.
fn collect_values(node: &Rc<TreeNode>) -> Vec<String> {
    todo!()
}

/// Demonstrates shared ownership: creates a "diamond" shaped graph where
/// a shared node has two parents. Returns a tuple of:
/// (reference count of shared node, values from parent1, values from parent2)
///
/// Structure:
///     parent1    parent2
///        \        /
///        shared_child
///
/// The shared_child should have value "shared".
/// parent1 should have value "parent1" with shared_child as its only child.
/// parent2 should have value "parent2" with shared_child as its only child.
fn build_diamond() -> (usize, Vec<String>, Vec<String>) {
    todo!()
}

/// Creates a list of Rc-wrapped strings and demonstrates how cloning
/// Rc does NOT deep-copy the data. Returns a vector of reference counts
/// for each original Rc after all clones are made.
///
/// Steps:
/// 1. Create Rc<String> for each input word
/// 2. Clone each Rc into a separate "clones" vector
/// 3. Return the reference count of each original Rc (should all be 2)
fn clone_counts(words: &[&str]) -> Vec<usize> {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_leaf() {
        let leaf = make_leaf("hello");
        assert_eq!(leaf.value, "hello");
        assert!(leaf.children.is_empty());
        assert_eq!(ref_count(&leaf), 1);
    }

    #[test]
    fn test_make_node_with_children() {
        let child1 = make_leaf("child1");
        let child2 = make_leaf("child2");
        let parent = make_node("parent", vec![Rc::clone(&child1), Rc::clone(&child2)]);

        assert_eq!(parent.value, "parent");
        assert_eq!(parent.children.len(), 2);
        // child1 and child2 are each owned by: original binding + parent's children vec
        assert_eq!(ref_count(&child1), 2);
        assert_eq!(ref_count(&child2), 2);
    }

    #[test]
    fn test_collect_values() {
        let tree = make_node(
            "root",
            vec![
                make_node("a", vec![make_leaf("a1"), make_leaf("a2")]),
                make_leaf("b"),
            ],
        );
        assert_eq!(
            collect_values(&tree),
            vec!["root", "a", "a1", "a2", "b"]
        );
    }

    #[test]
    fn test_collect_values_single() {
        let leaf = make_leaf("only");
        assert_eq!(collect_values(&leaf), vec!["only"]);
    }

    #[test]
    fn test_build_diamond() {
        let (shared_count, parent1_values, parent2_values) = build_diamond();

        // The shared node is owned by: parent1's children + parent2's children + local binding
        // (at least 3, but the function might drop the local -- minimum 2 from parents)
        assert!(shared_count >= 2, "shared node should have at least 2 owners");

        assert_eq!(parent1_values, vec!["parent1", "shared"]);
        assert_eq!(parent2_values, vec!["parent2", "shared"]);
    }

    #[test]
    fn test_clone_counts() {
        let counts = clone_counts(&["alpha", "beta", "gamma"]);
        assert_eq!(counts, vec![2, 2, 2]);
    }

    #[test]
    fn test_clone_counts_empty() {
        let counts = clone_counts(&[]);
        assert!(counts.is_empty());
    }

    #[test]
    fn test_rc_drop_decrements_count() {
        let node = make_leaf("temp");
        assert_eq!(ref_count(&node), 1);

        let clone1 = Rc::clone(&node);
        assert_eq!(ref_count(&node), 2);

        let clone2 = Rc::clone(&node);
        assert_eq!(ref_count(&node), 3);

        drop(clone1);
        assert_eq!(ref_count(&node), 2);

        drop(clone2);
        assert_eq!(ref_count(&node), 1);
    }
}
