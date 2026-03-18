// Exercise: The Error Handling Gauntlet (Boss Battle)
// Type: boss_battle
// Difficulty: hard
//
// === BOSS BATTLE ===
// Build a mini CSV parser that handles multiple error types.
// You'll parse CSV data representing student grades, validate the data,
// and compute statistics. Every step can fail in different ways.
//
// Implement the types and functions to make all tests pass.

use std::fmt;
use std::num::ParseFloatError;

// TODO: Define a `CsvError` enum with these variants:
//   - EmptyInput                     — no data provided
//   - InvalidHeader(String)          — header row doesn't match expected format
//   - InvalidRowLength { expected: usize, got: usize, row: usize }
//                                    — a row has wrong number of columns
//   - ParseError { row: usize, column: String, source: ParseFloatError }
//                                    — couldn't parse a numeric field
//   - ValidationError(String)        — data validation failed

// TODO: Implement fmt::Display for CsvError with descriptive messages.

// A parsed student record.
#[derive(Debug, Clone, PartialEq)]
struct Student {
    name: String,
    grade: f64,
}

// TODO: Implement these functions:
//
// - parse_csv(input: &str) -> Result<Vec<Student>, CsvError>
//       Parse CSV text with header "name,grade" into Student records.
//       Steps:
//       1. Return EmptyInput if input is empty or whitespace-only.
//       2. Split into lines, first line is the header.
//       3. Validate header is "name,grade" (case-insensitive), else InvalidHeader.
//       4. For each subsequent non-empty line:
//          a. Split by comma. If not exactly 2 fields, return InvalidRowLength.
//          b. Parse the grade field as f64. If it fails, return ParseError.
//          c. Validate grade is between 0.0 and 100.0, else ValidationError.
//       5. Return the Vec of Students.
//
// - class_average(students: &[Student]) -> Result<f64, CsvError>
//       Compute the average grade. Return EmptyInput if slice is empty.
//
// - top_students(students: &[Student], threshold: f64) -> Vec<&Student>
//       Return references to students with grade >= threshold, sorted by
//       grade descending.
//
// - parse_and_summarize(input: &str) -> Result<String, CsvError>
//       Parse the CSV, compute average, find top students (>= 90),
//       and return a summary string like:
//       "3 students, average: 85.00, top students: Alice, Bob"
//       If no top students: "3 students, average: 75.00, top students: none"

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_valid() {
        let input = "name,grade\nAlice,95.0\nBob,87.5\nCharlie,72.0";
        let students = parse_csv(input).unwrap();
        assert_eq!(students.len(), 3);
        assert_eq!(students[0].name, "Alice");
        assert!((students[0].grade - 95.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_parse_csv_empty() {
        assert!(matches!(parse_csv(""), Err(CsvError::EmptyInput)));
        assert!(matches!(parse_csv("   "), Err(CsvError::EmptyInput)));
    }

    #[test]
    fn test_parse_csv_bad_header() {
        let input = "student,score\nAlice,95";
        assert!(matches!(parse_csv(input), Err(CsvError::InvalidHeader(_))));
    }

    #[test]
    fn test_parse_csv_wrong_columns() {
        let input = "name,grade\nAlice,95,extra";
        let err = parse_csv(input).unwrap_err();
        if let CsvError::InvalidRowLength { expected, got, row } = err {
            assert_eq!(expected, 2);
            assert_eq!(got, 3);
            assert_eq!(row, 2);
        } else {
            panic!("Expected InvalidRowLength");
        }
    }

    #[test]
    fn test_parse_csv_bad_grade() {
        let input = "name,grade\nAlice,abc";
        let err = parse_csv(input).unwrap_err();
        assert!(matches!(err, CsvError::ParseError { row: 2, .. }));
    }

    #[test]
    fn test_parse_csv_grade_out_of_range() {
        let input = "name,grade\nAlice,150.0";
        assert!(matches!(parse_csv(input), Err(CsvError::ValidationError(_))));
    }

    #[test]
    fn test_parse_csv_negative_grade() {
        let input = "name,grade\nAlice,-5.0";
        assert!(matches!(parse_csv(input), Err(CsvError::ValidationError(_))));
    }

    #[test]
    fn test_class_average() {
        let students = vec![
            Student { name: "A".into(), grade: 80.0 },
            Student { name: "B".into(), grade: 90.0 },
            Student { name: "C".into(), grade: 70.0 },
        ];
        let avg = class_average(&students).unwrap();
        assert!((avg - 80.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_class_average_empty() {
        let students: Vec<Student> = vec![];
        assert!(matches!(class_average(&students), Err(CsvError::EmptyInput)));
    }

    #[test]
    fn test_top_students() {
        let students = vec![
            Student { name: "A".into(), grade: 95.0 },
            Student { name: "B".into(), grade: 70.0 },
            Student { name: "C".into(), grade: 92.0 },
            Student { name: "D".into(), grade: 88.0 },
        ];
        let top = top_students(&students, 90.0);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].name, "A"); // 95 first (descending)
        assert_eq!(top[1].name, "C"); // 92 second
    }

    #[test]
    fn test_top_students_none() {
        let students = vec![
            Student { name: "A".into(), grade: 60.0 },
        ];
        let top = top_students(&students, 90.0);
        assert!(top.is_empty());
    }

    #[test]
    fn test_parse_and_summarize() {
        let input = "name,grade\nAlice,95.0\nBob,92.0\nCharlie,72.0";
        let summary = parse_and_summarize(input).unwrap();
        assert!(summary.contains("3 students"));
        assert!(summary.contains("Alice"));
        assert!(summary.contains("Bob"));
        assert!(!summary.contains("Charlie")); // Charlie < 90
    }

    #[test]
    fn test_parse_and_summarize_no_top() {
        let input = "name,grade\nAlice,70.0\nBob,65.0";
        let summary = parse_and_summarize(input).unwrap();
        assert!(summary.contains("none"));
    }
}
