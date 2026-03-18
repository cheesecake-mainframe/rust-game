// Exercise: Defining Structs
// Type: fix_compiler_error
// Difficulty: intermediate
//
// Structs let you group related data together — like a Python class with
// only attributes (no methods yet). Each field has a name and a type.
//
// By default, struct fields are PRIVATE to the module they're defined in.
// To access them from outside, you need `pub`. Also, field names must
// match exactly when you construct or access them.
//
// Fix all the compiler errors below.

mod animal {
    struct Dog {
        name: String,
        age: u8,
        is_good_boy: bool,
    }

    impl Dog {
        pub fn new(name: &str, age: u8) -> Dog {
            Dog {
                name: name.to_string(),
                age,
                is_good_boy: true,
            }
        }
    }
}

fn main() {
    // TODO: Fix error — Dog and its fields are not accessible from here.
    let buddy = animal::Dog::new("Buddy", 3);

    // TODO: Fix error — field names must match the struct definition.
    println!("{} is {} years old", buddy.name, buddy.years);

    // TODO: Fix error — can't access private field.
    if buddy.is_good_boy {
        println!("{} is a good boy!", buddy.name);
    }

    // TODO: Fix error — missing field in struct construction.
    let puppy = animal::Dog {
        name: String::from("Rex"),
        age: 1,
    };
    println!("{} is {} years old, good boy: {}", puppy.name, puppy.age, puppy.is_good_boy);
}
