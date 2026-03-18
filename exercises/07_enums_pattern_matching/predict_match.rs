// ========================================
// Exercise: Predict the Match
// ========================================
// Type: Reverse Engineering (Multiple Choice)
// Difficulty: Intermediate
// Module: 07 - Enums & Pattern Matching
//
// CONCEPT:
// Read the code below carefully. Trace through each match arm
// to determine which branch executes and what gets printed.
//
// YOUR TASK:
// Select the correct output in the TUI multiple-choice screen.
// Use 'v' to open the answer selection.
// ========================================

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter: {}", state);
            25
        }
    }
}

fn main() {
    let coins = vec![
        Coin::Quarter("Alaska".to_string()),
        Coin::Penny,
        Coin::Dime,
    ];

    let mut total = 0;
    for coin in &coins {
        total += value_in_cents(coin);
    }
    println!("Total: {} cents", total);
}
