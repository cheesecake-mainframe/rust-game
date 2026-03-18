mod animal {
    pub struct Dog {
        pub name: String,
        pub age: u8,
        pub is_good_boy: bool,
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
    let buddy = animal::Dog::new("Buddy", 3);

    println!("{} is {} years old", buddy.name, buddy.age);

    if buddy.is_good_boy {
        println!("{} is a good boy!", buddy.name);
    }

    let puppy = animal::Dog {
        name: String::from("Rex"),
        age: 1,
        is_good_boy: true,
    };
    println!("{} is {} years old, good boy: {}", puppy.name, puppy.age, puppy.is_good_boy);
}
