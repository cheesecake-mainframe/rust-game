// Solution: Predict the Match
//
// The coins are iterated in order:
// 1. Quarter("Alaska") → prints "State quarter: Alaska", returns 25
// 2. Penny → prints "Lucky penny!", returns 1
// 3. Dime → no print, returns 10
// Total = 25 + 1 + 10 = 36
// Then prints "Total: 36 cents"
//
// Correct answer: A
// "State quarter: Alaska\nLucky penny!\nTotal: 36 cents"

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
