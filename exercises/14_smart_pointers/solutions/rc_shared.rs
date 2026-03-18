// ========================================
// Solution: Rc Shared Ownership
// ========================================

use std::rc::Rc;

#[derive(Debug)]
struct TreeNode {
    value: String,
    children: Vec<Rc<TreeNode>>,
}

/// Wraps a new TreeNode in Rc for shared ownership.
fn make_node(value: &str, children: Vec<Rc<TreeNode>>) -> Rc<TreeNode> {
    Rc::new(TreeNode {
        value: value.to_string(),
        children,
    })
}

/// A leaf is just a node with no children.
fn make_leaf(value: &str) -> Rc<TreeNode> {
    Rc::new(TreeNode {
        value: value.to_string(),
        children: Vec::new(),
    })
}

/// Rc::strong_count tells you how many Rc pointers share ownership.
fn ref_count(node: &Rc<TreeNode>) -> usize {
    Rc::strong_count(node)
}

/// Depth-first traversal: visit root first, then recurse into children.
fn collect_values(node: &Rc<TreeNode>) -> Vec<String> {
    let mut result = vec![node.value.clone()];
    for child in &node.children {
        result.extend(collect_values(child));
    }
    result
}

/// Demonstrates shared ownership with a diamond-shaped graph.
/// The shared_child node is owned by both parent1 and parent2.
fn build_diamond() -> (usize, Vec<String>, Vec<String>) {
    // Create the shared child node
    let shared_child = make_leaf("shared");

    // Both parents get an Rc::clone of the shared child.
    // Rc::clone just increments the reference count -- no data is copied.
    let parent1 = make_node("parent1", vec![Rc::clone(&shared_child)]);
    let parent2 = make_node("parent2", vec![Rc::clone(&shared_child)]);

    // shared_child is now owned by: shared_child binding + parent1 + parent2 = 3
    let count = ref_count(&shared_child);

    let p1_values = collect_values(&parent1);
    let p2_values = collect_values(&parent2);

    (count, p1_values, p2_values)
}

/// Creates Rc-wrapped strings, clones each one, and returns the reference counts.
fn clone_counts(words: &[&str]) -> Vec<usize> {
    // Create Rc<String> for each word
    let originals: Vec<Rc<String>> = words
        .iter()
        .map(|w| Rc::new(w.to_string()))
        .collect();

    // Clone each Rc (cheap -- just increments refcount)
    let _clones: Vec<Rc<String>> = originals
        .iter()
        .map(|rc| Rc::clone(rc))
        .collect();

    // Each original now has refcount 2 (original + clone)
    originals
        .iter()
        .map(|rc| Rc::strong_count(rc))
        .collect()
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

        assert!(shared_count >= 2);

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
