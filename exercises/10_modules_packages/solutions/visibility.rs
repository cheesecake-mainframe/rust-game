mod restaurant {
    pub(super) mod kitchen {
        pub struct Meal {
            pub name: String,
            pub calories: u32,
            secret_ingredient: String,
        }

        impl Meal {
            pub fn new(name: &str, calories: u32, secret: &str) -> Meal {
                Meal {
                    name: name.to_string(),
                    calories,
                    secret_ingredient: secret.to_string(),
                }
            }

            pub fn describe(&self) -> String {
                format!("{} ({} cal)", self.name, self.calories)
            }
        }

        pub fn prepare_meal(order: &str) -> Meal {
            match order {
                "burger" => Meal::new("Burger", 800, "special sauce"),
                "salad" => Meal::new("Salad", 350, "truffle oil"),
                _ => Meal::new("Mystery Dish", 500, "love"),
            }
        }
    }

    mod front_of_house {
        fn take_order(order: &str) -> String {
            let meal = super::kitchen::prepare_meal(order);
            format!("Order ready: {}", meal.describe())
        }

        pub fn seat_customer() {
            println!("Customer seated!");
            let order = take_order("burger");
            println!("{}", order);
        }
    }

    pub fn open_restaurant() {
        front_of_house::seat_customer();
    }
}

fn main() {
    restaurant::open_restaurant();
}
