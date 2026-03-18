// ========================================
// Exercise: Zero Copy Optimization
// ========================================
// Difficulty: Advanced
// Module: 13 - Closures & Iterators
//
// CONCEPT:
// Unnecessary .clone() calls waste memory and CPU. In many cases, you can
// use references (&T) and string slices (&str) instead of cloning owned
// data. This is especially important in performance-critical code.
//
// YOUR TASK:
// This code works correctly, but it uses .clone() everywhere unnecessarily.
// Refactor ALL functions to use references instead of cloning.
//
// CUSTOM CHECK: Your solution must NOT contain ".clone()" anywhere!
//
// RULES:
// - All tests must still pass
// - No .clone() calls allowed
// - You may change function signatures to use references
// - The data structures can stay the same
// ========================================

#[derive(Debug, PartialEq)]
struct Employee {
    name: String,
    department: String,
    salary: u64,
}

/// Returns the names of all employees in the given department.
fn employees_in_department(employees: &[Employee], department: &str) -> Vec<String> {
    employees
        .iter()
        .filter(|e| e.department == department)
        .map(|e| e.name.clone())
        .collect()
}

/// Returns the name of the highest-paid employee.
fn highest_paid_name(employees: &[Employee]) -> String {
    let best = employees
        .iter()
        .max_by_key(|e| e.salary)
        .unwrap();
    best.name.clone()
}

/// Returns a summary string for each employee: "Name (Department)"
fn employee_summaries(employees: &[Employee]) -> Vec<String> {
    employees
        .iter()
        .map(|e| {
            let name = e.name.clone();
            let dept = e.department.clone();
            format!("{} ({})", name, dept)
        })
        .collect()
}

/// Returns the total salary of employees whose names start with the given prefix.
fn total_salary_for_prefix(employees: &[Employee], prefix: &str) -> u64 {
    employees
        .iter()
        .filter(|e| {
            let name = e.name.clone();
            name.starts_with(prefix)
        })
        .map(|e| e.salary)
        .sum()
}

/// Returns a vector of (department, count) pairs.
fn department_counts(employees: &[Employee]) -> Vec<(String, usize)> {
    let mut departments: Vec<String> = employees
        .iter()
        .map(|e| e.department.clone())
        .collect();
    departments.sort();
    departments.dedup();

    departments
        .iter()
        .map(|dept| {
            let dept_clone = dept.clone();
            let count = employees
                .iter()
                .filter(|e| e.department.clone() == dept_clone)
                .count();
            (dept_clone, count)
        })
        .collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_employees() -> Vec<Employee> {
        vec![
            Employee {
                name: String::from("Alice"),
                department: String::from("Engineering"),
                salary: 120000,
            },
            Employee {
                name: String::from("Bob"),
                department: String::from("Engineering"),
                salary: 110000,
            },
            Employee {
                name: String::from("Carol"),
                department: String::from("Marketing"),
                salary: 95000,
            },
            Employee {
                name: String::from("Dave"),
                department: String::from("Marketing"),
                salary: 90000,
            },
            Employee {
                name: String::from("Alice Smith"),
                department: String::from("Sales"),
                salary: 85000,
            },
        ]
    }

    #[test]
    fn test_employees_in_department() {
        let emps = sample_employees();
        let eng = employees_in_department(&emps, "Engineering");
        assert_eq!(eng, vec!["Alice", "Bob"]);
    }

    #[test]
    fn test_employees_in_department_empty() {
        let emps = sample_employees();
        let hr = employees_in_department(&emps, "HR");
        assert!(hr.is_empty());
    }

    #[test]
    fn test_highest_paid_name() {
        let emps = sample_employees();
        assert_eq!(highest_paid_name(&emps), "Alice");
    }

    #[test]
    fn test_employee_summaries() {
        let emps = sample_employees();
        let summaries = employee_summaries(&emps);
        assert_eq!(summaries[0], "Alice (Engineering)");
        assert_eq!(summaries[2], "Carol (Marketing)");
        assert_eq!(summaries.len(), 5);
    }

    #[test]
    fn test_total_salary_for_prefix() {
        let emps = sample_employees();
        // "Alice" + "Alice Smith" = 120000 + 85000 = 205000
        assert_eq!(total_salary_for_prefix(&emps, "Alice"), 205000);
        assert_eq!(total_salary_for_prefix(&emps, "Bob"), 110000);
        assert_eq!(total_salary_for_prefix(&emps, "Z"), 0);
    }

    #[test]
    fn test_department_counts() {
        let emps = sample_employees();
        let counts = department_counts(&emps);
        // Sorted alphabetically: Engineering(2), Marketing(2), Sales(1)
        assert_eq!(counts.len(), 3);
        assert!(counts.contains(&(String::from("Engineering"), 2)));
        assert!(counts.contains(&(String::from("Marketing"), 2)));
        assert!(counts.contains(&(String::from("Sales"), 1)));
    }
}
