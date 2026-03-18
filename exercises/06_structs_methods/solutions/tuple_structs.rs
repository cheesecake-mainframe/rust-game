#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Meters(f64);

#[derive(Debug)]
struct Seconds(f64);

fn calculate_speed(distance: Meters, time: Seconds) -> f64 {
    distance.0 / time.0
}

fn main() {
    let red = Color(255, 0, 0);

    println!("Red: ({}, {}, {})", red.0, red.1, red.2);

    println!("Color: {:?}", red);

    let distance = Meters(100.0);
    let time = Seconds(9.58);

    let speed = calculate_speed(distance, time);
    println!("Speed: {:.2} m/s", speed);

    let m1 = Meters(5.0);
    let m2 = Meters(3.0);
    let sum = m1.0 + m2.0;
    let result = Meters(sum);
    println!("This should be meters: {:?}", result);
}
