use coffee::Coffee;
pub use coffee::CoffeeType;
#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    coffee: Coffee,
}

mod coffee {
    #[derive(Debug)]
    pub struct Coffee {
        pub coffee_type: CoffeeType,
        beans: String,
    }
    #[derive(Debug)]
    pub enum CoffeeType {
        Filter,
        Espresso,
        Latte,
        Americano,
    }
    const BEANS_AVAILABLE: [&str; 2] = ["Arabica", "Robusta"];

    impl Coffee {
        fn check_bean_availability(beans: &str) -> bool {
            for bean in BEANS_AVAILABLE {
                if bean == beans {
                    return true;
                }
            }
            false
        }
        pub fn change_beans(&mut self, beans: &str) -> bool {
            if Coffee::check_bean_availability(beans) {
                self.beans = String::from(beans);
                return true;
            }
            false
        }

        pub fn classic_american() -> Coffee {
            Coffee {
                coffee_type: super::CoffeeType::Americano,
                beans: String::from("Arabica"),
            }
        }
    }
}


impl Breakfast {
    pub fn american(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            coffee: Coffee::classic_american(),
        }
    }
    pub fn request_different_coffee(&mut self, coffee_type: CoffeeType, beans: &str) -> bool {
        self.coffee.coffee_type = coffee_type;
        self.coffee.change_beans(beans)
    }
}

fn fix_incorrect_order() {
    cook_order(); // can access because in the same level, siblings
    deliver_order(); // can access super and it's contents because going upward in the tree is not limited
}

fn cook_order() {}

fn deliver_order() {}