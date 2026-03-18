use std::fmt;
use std::num::ParseFloatError;

#[derive(Debug)]
enum CsvError {
    EmptyInput,
    InvalidHeader(String),
    InvalidRowLength { expected: usize, got: usize, row: usize },
    ParseError { row: usize, column: String, source: ParseFloatError },
    ValidationError(String),
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsvError::EmptyInput => write!(f, "Input is empty"),
            CsvError::InvalidHeader(h) => {
                write!(f, "Invalid header: '{}', expected 'name,grade'", h)
            }
            CsvError::InvalidRowLength { expected, got, row } => {
                write!(f, "Row {}: expected {} columns, got {}", row, expected, got)
            }
            CsvError::ParseError { row, column, source } => {
                write!(f, "Row {}: failed to parse column '{}': {}", row, column, source)
            }
            CsvError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Student {
    name: String,
    grade: f64,
}

fn parse_csv(input: &str) -> Result<Vec<Student>, CsvError> {
    if input.trim().is_empty() {
        return Err(CsvError::EmptyInput);
    }

    let mut lines = input.lines();

    let header = lines.next().ok_or(CsvError::EmptyInput)?;
    if header.trim().to_lowercase() != "name,grade" {
        return Err(CsvError::InvalidHeader(header.to_string()));
    }

    let mut students = Vec::new();
    for (i, line) in lines.enumerate() {
        let row_num = i + 2; // 1-indexed, header is row 1
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != 2 {
            return Err(CsvError::InvalidRowLength {
                expected: 2,
                got: fields.len(),
                row: row_num,
            });
        }

        let name = fields[0].trim().to_string();
        let grade: f64 = fields[1].trim().parse().map_err(|e| CsvError::ParseError {
            row: row_num,
            column: "grade".to_string(),
            source: e,
        })?;

        if grade < 0.0 || grade > 100.0 {
            return Err(CsvError::ValidationError(format!(
                "Grade {} for '{}' is out of range 0-100",
                grade, name
            )));
        }

        students.push(Student { name, grade });
    }

    Ok(students)
}

fn class_average(students: &[Student]) -> Result<f64, CsvError> {
    if students.is_empty() {
        return Err(CsvError::EmptyInput);
    }
    let sum: f64 = students.iter().map(|s| s.grade).sum();
    Ok(sum / students.len() as f64)
}

fn top_students(students: &[Student], threshold: f64) -> Vec<&Student> {
    let mut top: Vec<&Student> = students.iter().filter(|s| s.grade >= threshold).collect();
    top.sort_by(|a, b| b.grade.partial_cmp(&a.grade).unwrap());
    top
}

fn parse_and_summarize(input: &str) -> Result<String, CsvError> {
    let students = parse_csv(input)?;
    let avg = class_average(&students)?;
    let top = top_students(&students, 90.0);

    let top_names = if top.is_empty() {
        "none".to_string()
    } else {
        top.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join(", ")
    };

    Ok(format!(
        "{} students, average: {:.2}, top students: {}",
        students.len(),
        avg,
        top_names
    ))
}

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
        assert_eq!(top[0].name, "A");
        assert_eq!(top[1].name, "C");
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
        assert!(!summary.contains("Charlie"));
    }

    #[test]
    fn test_parse_and_summarize_no_top() {
        let input = "name,grade\nAlice,70.0\nBob,65.0";
        let summary = parse_and_summarize(input).unwrap();
        assert!(summary.contains("none"));
    }
}
