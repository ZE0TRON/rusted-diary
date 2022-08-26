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