// Exercise: Visibility
// Type: fix_compiler_error
// Difficulty: intermediate
//
// In Rust, everything is PRIVATE by default. To make something
// accessible from outside its module, you must mark it `pub`.
//
// Visibility rules:
//   - `pub` — accessible from anywhere
//   - `pub(crate)` — accessible within the current crate only
//   - `pub(super)` — accessible from the parent module
//   - (no modifier) — private to the current module
//
// Fix all the compiler errors by adjusting visibility modifiers.

mod restaurant {
    mod kitchen {
        struct Meal {
            name: String,
            calories: u32,
            secret_ingredient: String,
        }

        impl Meal {
            fn new(name: &str, calories: u32, secret: &str) -> Meal {
                Meal {
                    name: name.to_string(),
                    calories,
                    secret_ingredient: secret.to_string(),
                }
            }

            fn describe(&self) -> String {
                format!("{} ({} cal)", self.name, self.calories)
            }
        }

        fn prepare_meal(order: &str) -> Meal {
            match order {
                "burger" => Meal::new("Burger", 800, "special sauce"),
                "salad" => Meal::new("Salad", 350, "truffle oil"),
                _ => Meal::new("Mystery Dish", 500, "love"),
            }
        }
    }

    mod front_of_house {
        fn take_order(order: &str) -> String {
            // TODO: Fix — can't access kitchen::prepare_meal (it's private)
            let meal = super::kitchen::prepare_meal(order);

            // TODO: Fix — can't access meal.name (it's private)
            format!("Order ready: {}", meal.describe())
        }

        pub fn seat_customer() {
            println!("Customer seated!");
            let order = take_order("burger");
            println!("{}", order);
        }
    }

    pub fn open_restaurant() {
        // TODO: Fix — can't access front_of_house::seat_customer
        front_of_house::seat_customer();
    }
}

fn main() {
    // TODO: Fix — can't access restaurant::open_restaurant
    restaurant::open_restaurant();
}
