#[derive(Debug)]
struct Tamagotchi {
    name : String,
    hunger: u8,
    sleep: u8,
    social: u8,
    alive: bool
}

fn cap_at(value: u8, cap: u8) -> u8 {
    if value < cap {value} else {cap}
}

impl Tamagotchi{
    fn feed(&mut self) -> u8 {
        self.hunger +=30;
        self.hunger = cap_at(self.hunger, 100);
        return self.hunger;
    }

    fn sleep(&mut self, hours: u8) {
        self.sleep += (if hours > 24 {24} else {hours}) * 4;
        self.sleep = cap_at(self.sleep, 100);
    }

    fn socialize(&mut self) {
        self.social+=20;
        self.social = cap_at(self.social, 100);
    }

    fn talk(&mut self) {
        println!("Hi");
        if self.hunger < 30 {
            println!("I am hungry");
        }
        if self.sleep < 30 {
            println!("I need sleep");
        }
        println!("Thanks for talking with me");
        self.socialize();
        if self.social < 30 {
            println!("Can we talk a bit more ?");
        }
    }
    
    fn stats(&self) {
        println!("Hunger : {} ", self.hunger);
        println!("Sleep : {}", self.sleep);
        println!("Social : {}", self.social);
    }

    fn create(name : String) -> Self {
        println!("Hi, I am  {name}. Can you take care of me ?");
        Self { name, hunger: 50, sleep: 50, social: 50, alive: true}
    }

    fn time_tick(&mut self) -> bool {
        if self.hunger <= 10 || self.sleep <= 10 || self.social <= 10 {
            self.alive = false;
            return false;
        }
        self.hunger -=10;
        self.sleep -=10;
        self.social -=10;
        return true;
    }

}

fn main() {
    let mut simp = Tamagotchi::create(String::from("Simp"));
    simp.stats();
    simp.talk();
    simp.feed();
    simp.time_tick();
    simp.stats();
    // TODO loop and take input and match with action
}
