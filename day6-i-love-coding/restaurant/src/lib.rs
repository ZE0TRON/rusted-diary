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

mod back_of_house;

use front_of_house::hosting as reception;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // need the pub mod hosting and pub fn add_to_waitlist

    reception::add_to_waitlist(); // same as above
}


use back_of_house::{Breakfast, CoffeeType};

pub fn order_breakfast() {
    let mut breakfast = Breakfast::american("Wheat");
    breakfast.toast = String::from("Rye"); // I changed my mind want different bread
    breakfast.request_different_coffee(CoffeeType::Espresso, "Robusta");
    println!("My Breakfast is {:?}", breakfast);
}

// Nested with self 
// use std::io::{self, Write};
// Glob operator to bring everything to the scope
// use std::collections::*;