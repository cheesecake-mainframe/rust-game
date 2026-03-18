// ========================================
// Boss Battle: Generic Collection with Trait-Based Filtering & Sorting
// ========================================
// Difficulty: Hard
// Module: 11 - Generics & Traits
//
// CONCEPT:
// This boss battle combines generics, trait definitions, trait bounds,
// and default implementations into a real-world-style challenge.
//
// YOUR TASK:
// Build a `SmartCollection<T>` that can store, filter, and sort items
// based on traits. You need to:
//
// 1. Define a `Prioritized` trait with:
//    - fn priority(&self) -> u32
//    - fn label(&self) -> &str
//    - fn is_urgent(&self) -> bool (default: priority > 7)
//
// 2. Define a `Categorized` trait with:
//    - fn category(&self) -> &str
//
// 3. Implement `SmartCollection<T>` with these methods:
//    - new() -> Self
//    - add(item: T)
//    - len() -> usize
//    - is_empty() -> bool
//    - filter_by_priority(min_priority: u32) -> Vec<&T>
//      (where T: Prioritized)
//    - urgent_items() -> Vec<&T>
//      (where T: Prioritized)
//    - items_in_category(category: &str) -> Vec<&T>
//      (where T: Categorized)
//    - sorted_by_priority() -> Vec<&T>
//      (where T: Prioritized, sorted descending by priority)
//    - summary() -> String
//      (where T: Prioritized + Categorized, returns formatted summary)
//
// 4. Implement the traits for `Task` and `Bug` structs (provided below).
//
// HINTS:
// - SmartCollection is just a wrapper around Vec<T>
// - Use separate impl blocks with different trait bounds for different methods
// - sorted_by_priority should return highest priority first
// - summary format: "Collection: N items, M urgent"
// ========================================

/// A task item with a name, priority level, and category.
struct Task {
    name: String,
    priority_level: u32,
    area: String,
}

/// A bug report with severity and component.
struct Bug {
    title: String,
    severity: u32,
    component: String,
}

// TODO: Define the `Prioritized` trait.

// TODO: Define the `Categorized` trait.

// TODO: Implement `Prioritized` for `Task`.
// priority() -> self.priority_level
// label() -> &self.name

// TODO: Implement `Categorized` for `Task`.
// category() -> &self.area

// TODO: Implement `Prioritized` for `Bug`.
// priority() -> self.severity
// label() -> &self.title

// TODO: Implement `Categorized` for `Bug`.
// category() -> &self.component

// TODO: Define `SmartCollection<T>` struct and implement all methods.

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tasks() -> SmartCollection<Task> {
        let mut col = SmartCollection::new();
        col.add(Task {
            name: String::from("Write docs"),
            priority_level: 3,
            area: String::from("documentation"),
        });
        col.add(Task {
            name: String::from("Fix login bug"),
            priority_level: 9,
            area: String::from("backend"),
        });
        col.add(Task {
            name: String::from("Update CSS"),
            priority_level: 5,
            area: String::from("frontend"),
        });
        col.add(Task {
            name: String::from("Deploy hotfix"),
            priority_level: 10,
            area: String::from("backend"),
        });
        col.add(Task {
            name: String::from("Code review"),
            priority_level: 7,
            area: String::from("backend"),
        });
        col
    }

    fn sample_bugs() -> SmartCollection<Bug> {
        let mut col = SmartCollection::new();
        col.add(Bug {
            title: String::from("Crash on startup"),
            severity: 10,
            component: String::from("core"),
        });
        col.add(Bug {
            title: String::from("Typo in footer"),
            severity: 2,
            component: String::from("ui"),
        });
        col.add(Bug {
            title: String::from("Slow query"),
            severity: 8,
            component: String::from("database"),
        });
        col
    }

    #[test]
    fn test_new_and_len() {
        let col: SmartCollection<Task> = SmartCollection::new();
        assert!(col.is_empty());
        assert_eq!(col.len(), 0);
    }

    #[test]
    fn test_add_and_len() {
        let col = sample_tasks();
        assert_eq!(col.len(), 5);
        assert!(!col.is_empty());
    }

    #[test]
    fn test_filter_by_priority() {
        let col = sample_tasks();
        let high = col.filter_by_priority(8);
        assert_eq!(high.len(), 2); // Fix login bug (9) and Deploy hotfix (10)
    }

    #[test]
    fn test_urgent_items() {
        let col = sample_tasks();
        let urgent = col.urgent_items();
        // Urgent = priority > 7, so: Fix login bug (9), Deploy hotfix (10)
        assert_eq!(urgent.len(), 2);
        for item in &urgent {
            assert!(item.priority() > 7);
        }
    }

    #[test]
    fn test_items_in_category() {
        let col = sample_tasks();
        let backend = col.items_in_category("backend");
        assert_eq!(backend.len(), 3); // Fix login bug, Deploy hotfix, Code review
    }

    #[test]
    fn test_sorted_by_priority() {
        let col = sample_tasks();
        let sorted = col.sorted_by_priority();
        assert_eq!(sorted.len(), 5);
        // Should be descending: 10, 9, 7, 5, 3
        assert_eq!(sorted[0].priority(), 10);
        assert_eq!(sorted[1].priority(), 9);
        assert_eq!(sorted[2].priority(), 7);
        assert_eq!(sorted[3].priority(), 5);
        assert_eq!(sorted[4].priority(), 3);
    }

    #[test]
    fn test_summary() {
        let col = sample_tasks();
        let summary = col.summary();
        assert_eq!(summary, "Collection: 5 items, 2 urgent");
    }

    #[test]
    fn test_bugs_filter() {
        let col = sample_bugs();
        let critical = col.filter_by_priority(9);
        assert_eq!(critical.len(), 1);
        assert_eq!(critical[0].label(), "Crash on startup");
    }

    #[test]
    fn test_bugs_category() {
        let col = sample_bugs();
        let core_bugs = col.items_in_category("core");
        assert_eq!(core_bugs.len(), 1);
    }

    #[test]
    fn test_bugs_urgent() {
        let col = sample_bugs();
        let urgent = col.urgent_items();
        assert_eq!(urgent.len(), 2); // severity 10 and 8
    }

    #[test]
    fn test_bugs_summary() {
        let col = sample_bugs();
        assert_eq!(col.summary(), "Collection: 3 items, 2 urgent");
    }

    #[test]
    fn test_default_is_urgent() {
        let task = Task {
            name: String::from("Normal task"),
            priority_level: 7,
            area: String::from("misc"),
        };
        // priority 7 is NOT > 7, so not urgent
        assert!(!task.is_urgent());

        let task2 = Task {
            name: String::from("Hot task"),
            priority_level: 8,
            area: String::from("misc"),
        };
        assert!(task2.is_urgent());
    }
}
