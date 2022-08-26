use coffee::Coffee;
pub use coffee::CoffeeType;
#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    coffee: Coffee,
}

mod coffee;

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