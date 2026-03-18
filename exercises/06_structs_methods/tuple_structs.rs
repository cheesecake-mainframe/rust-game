// Exercise: Tuple Structs
// Type: fix_compiler_error
// Difficulty: intermediate
//
// Tuple structs are like regular structs but with unnamed fields,
// accessed by index (e.g., color.0, color.1).
//
// The "newtype pattern" wraps a single value in a struct to give it
// a distinct type — e.g., Meters(f64) vs Seconds(f64) prevents
// accidentally mixing up units.
//
// Fix all the compiler errors below.

// A color represented as RGB values (0-255).
struct Color(u8, u8, u8);

// Newtype wrappers for type safety.
struct Meters(f64);
struct Seconds(f64);

// TODO: Fix — this function should accept Meters and Seconds,
// but the types don't work correctly yet.
fn calculate_speed(distance: Meters, time: Seconds) -> f64 {
    distance / time
}

fn main() {
    // TODO: Fix — wrong syntax for constructing tuple struct.
    let red = Color { r: 255, g: 0, b: 0 };

    // TODO: Fix — wrong field access syntax for tuple struct.
    println!("Red: ({}, {}, {})", red.r, red.g, red.b);

    // TODO: Fix — Color is not printable.
    println!("Color: {:?}", red);

    let distance = Meters(100.0);
    let time = Seconds(9.58);

    let speed = calculate_speed(distance, time);
    println!("Speed: {:.2} m/s", speed);

    // TODO: Fix — can't add Meters to Seconds (they're different types).
    let wrong = Meters(5.0);
    let also_wrong = Seconds(3.0);
    let sum = wrong.0 + also_wrong.0;
    let result = Meters(sum);
    println!("This should be meters: {:?}", result);
}
