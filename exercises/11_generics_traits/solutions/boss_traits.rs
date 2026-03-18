// ========================================
// Solution: Boss Battle - Generic Collection with Trait-Based Filtering & Sorting
// ========================================

struct Task {
    name: String,
    priority_level: u32,
    area: String,
}

struct Bug {
    title: String,
    severity: u32,
    component: String,
}

trait Prioritized {
    fn priority(&self) -> u32;
    fn label(&self) -> &str;
    fn is_urgent(&self) -> bool {
        self.priority() > 7
    }
}

trait Categorized {
    fn category(&self) -> &str;
}

impl Prioritized for Task {
    fn priority(&self) -> u32 {
        self.priority_level
    }
    fn label(&self) -> &str {
        &self.name
    }
}

impl Categorized for Task {
    fn category(&self) -> &str {
        &self.area
    }
}

impl Prioritized for Bug {
    fn priority(&self) -> u32 {
        self.severity
    }
    fn label(&self) -> &str {
        &self.title
    }
}

impl Categorized for Bug {
    fn category(&self) -> &str {
        &self.component
    }
}

struct SmartCollection<T> {
    items: Vec<T>,
}

impl<T> SmartCollection<T> {
    fn new() -> Self {
        SmartCollection { items: Vec::new() }
    }

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T: Prioritized> SmartCollection<T> {
    fn filter_by_priority(&self, min_priority: u32) -> Vec<&T> {
        self.items
            .iter()
            .filter(|item| item.priority() >= min_priority)
            .collect()
    }

    fn urgent_items(&self) -> Vec<&T> {
        self.items
            .iter()
            .filter(|item| item.is_urgent())
            .collect()
    }

    fn sorted_by_priority(&self) -> Vec<&T> {
        let mut refs: Vec<&T> = self.items.iter().collect();
        refs.sort_by(|a, b| b.priority().cmp(&a.priority()));
        refs
    }
}

impl<T: Categorized> SmartCollection<T> {
    fn items_in_category<'a>(&'a self, category: &str) -> Vec<&'a T> {
        self.items
            .iter()
            .filter(|item| item.category() == category)
            .collect()
    }
}

impl<T: Prioritized + Categorized> SmartCollection<T> {
    fn summary(&self) -> String {
        let urgent_count = self.items.iter().filter(|item| item.is_urgent()).count();
        format!(
            "Collection: {} items, {} urgent",
            self.items.len(),
            urgent_count
        )
    }
}

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
        assert_eq!(high.len(), 2);
    }

    #[test]
    fn test_urgent_items() {
        let col = sample_tasks();
        let urgent = col.urgent_items();
        assert_eq!(urgent.len(), 2);
        for item in &urgent {
            assert!(item.priority() > 7);
        }
    }

    #[test]
    fn test_items_in_category() {
        let col = sample_tasks();
        let backend = col.items_in_category("backend");
        assert_eq!(backend.len(), 3);
    }

    #[test]
    fn test_sorted_by_priority() {
        let col = sample_tasks();
        let sorted = col.sorted_by_priority();
        assert_eq!(sorted.len(), 5);
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
        assert_eq!(urgent.len(), 2);
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
        assert!(!task.is_urgent());

        let task2 = Task {
            name: String::from("Hot task"),
            priority_level: 8,
            area: String::from("misc"),
        };
        assert!(task2.is_urgent());
    }
}
