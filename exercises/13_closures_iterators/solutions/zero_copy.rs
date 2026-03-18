// ========================================
// Solution: Zero Copy Optimization
// ========================================
// All .clone() calls have been removed. Functions return references
// or use references directly instead of cloning owned data.

#[derive(Debug, PartialEq)]
struct Employee {
    name: String,
    department: String,
    salary: u64,
}

/// Returns the names of all employees in the given department.
/// Changed: return &str references instead of cloned Strings.
fn employees_in_department<'a>(employees: &'a [Employee], department: &str) -> Vec<&'a str> {
    employees
        .iter()
        .filter(|e| e.department == department)
        .map(|e| e.name.as_str())
        .collect()
}

/// Returns the name of the highest-paid employee.
/// Changed: return &str reference instead of cloned String.
fn highest_paid_name(employees: &[Employee]) -> &str {
    let best = employees
        .iter()
        .max_by_key(|e| e.salary)
        .unwrap();
    &best.name
}

/// Returns a summary string for each employee: "Name (Department)"
/// Changed: use references directly in format!, no intermediate clones.
fn employee_summaries(employees: &[Employee]) -> Vec<String> {
    employees
        .iter()
        .map(|e| format!("{} ({})", e.name, e.department))
        .collect()
}

/// Returns the total salary of employees whose names start with the given prefix.
/// Changed: call starts_with directly on the &String, no clone needed.
fn total_salary_for_prefix(employees: &[Employee], prefix: &str) -> u64 {
    employees
        .iter()
        .filter(|e| e.name.starts_with(prefix))
        .map(|e| e.salary)
        .sum()
}

/// Returns a vector of (department, count) pairs.
/// Changed: use &str references for intermediate collection, only create
/// owned Strings at the final output.
fn department_counts(employees: &[Employee]) -> Vec<(String, usize)> {
    let mut departments: Vec<&str> = employees
        .iter()
        .map(|e| e.department.as_str())
        .collect();
    departments.sort();
    departments.dedup();

    departments
        .iter()
        .map(|&dept| {
            let count = employees
                .iter()
                .filter(|e| e.department == dept)
                .count();
            (String::from(dept), count)
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
        assert_eq!(total_salary_for_prefix(&emps, "Alice"), 205000);
        assert_eq!(total_salary_for_prefix(&emps, "Bob"), 110000);
        assert_eq!(total_salary_for_prefix(&emps, "Z"), 0);
    }

    #[test]
    fn test_department_counts() {
        let emps = sample_employees();
        let counts = department_counts(&emps);
        assert_eq!(counts.len(), 3);
        assert!(counts.contains(&(String::from("Engineering"), 2)));
        assert!(counts.contains(&(String::from("Marketing"), 2)));
        assert!(counts.contains(&(String::from("Sales"), 1)));
    }
}
