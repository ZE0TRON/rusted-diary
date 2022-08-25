mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        coffee: coffee::Coffee,
    }

    #[derive(Debug)]
    pub enum CoffeeType {
        Filter,
        Espresso,
        Latte,
        Americano,
    }

    mod coffee {
        #[derive(Debug)]
        pub struct Coffee {
            pub coffee_type: super::CoffeeType,
            beans: String,
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
                coffee: coffee::Coffee::classic_american(),
            }
        }
        pub fn request_different_coffee(&mut self, coffee_type: CoffeeType, beans: &str) -> bool {
            self.coffee.coffee_type = coffee_type;
            self.coffee.change_beans(beans)
        }
    }

    fn fix_incorrect_order() {
        cook_order(); // can access because in the same level, siblings
        super::deliver_order(); // can access super and it's contents because going upward in the tree is not limited
    }

    fn cook_order() {}
}

fn deliver_order() {}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // need the pub mod hosting and pub fn add_to_waitlist

    front_of_house::hosting::add_to_waitlist(); // same as above
}
pub fn order_breakfast() {
    let mut breakfast = back_of_house::Breakfast::american("Wheat");
    breakfast.toast = String::from("Rye"); // I changed my mind want different bread
    breakfast.request_different_coffee(back_of_house::CoffeeType::Espresso, "Robusta");
    println!("My Breakfast is {:?}", breakfast);
}
