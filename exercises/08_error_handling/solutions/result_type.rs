use std::num::ParseIntError;

fn parse_age(input: &str) -> Result<u32, ParseIntError> {
    let age = input.trim().parse::<u32>()?;
    Ok(age)
}

fn parse_and_double(input: &str) -> Result<u32, ParseIntError> {
    let n = input.parse::<u32>()?;
    Ok(n * 2)
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Cannot divide by zero"));
    }
    Ok(a / b)
}

fn process_input(input: &str) -> Result<String, String> {
    let number: i32 = input.parse().map_err(|e: ParseIntError| e.to_string())?;
    let result = divide(number, number - 1)?;
    Ok(format!("Result: {}", result))
}

fn main() {
    match parse_and_double("21") {
        Ok(val) => println!("Doubled: {}", val),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10, 3) {
        Ok(val) => println!("10 / 3 = {}", val),
        Err(e) => println!("Error: {}", e),
    }
}
