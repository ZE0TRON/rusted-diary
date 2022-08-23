use std::io;

#[derive(Debug)]
struct Tamagotchi {
    name : String,
    fed: u8,
    sleep: u8,
    social: u8,
    alive: bool
}

fn cap_at(value: u8, cap: u8) -> u8 {
    if value < cap {value} else {cap}
}

impl Tamagotchi{
    fn feed(&mut self) {
        self.fed +=40;
        self.fed = cap_at(self.fed, 100);
    }

    fn sleep(&mut self, hours: u8) {
        self.sleep += (if hours > 24 {24} else {hours}) * 13;
        self.sleep = cap_at(self.sleep, 100);
        for _ in 0..(hours/4) {
            self.time_tick();
        }
    }

    fn socialize(&mut self) {
        self.social+=40;
        self.social = cap_at(self.social, 100);
    }

    fn talk(self: &mut Self) {
        println!("Hi");
        if self.fed < 30 {
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
        println!("fed : {} ", self.fed);
        println!("Sleep : {}", self.sleep);
        println!("Social : {}", self.social);
    }

    fn create(name : String) -> Self {
        println!("Hi, I am  {name}. Can you take care of me ?");
        Self { name, fed: 50, sleep: 50, social: 50, alive: true}
    }

    fn time_tick(&mut self) -> bool {
        if self.fed <= 10 || self.sleep <= 10 || self.social <= 10 {
            self.alive = false;
            return false;
        }
        self.fed -=10;
        self.sleep -=10;
        self.social -=10;
        return true;
    }

}

fn parse_sleep_command(command_param: Option<&str>, simp : &mut Tamagotchi) -> bool {
    if let Some(time) = command_param {
        let time = u8::from_str_radix(time, 10);
        if let Some(time) =  time.ok() {
            simp.sleep(time);
            true
        } else { 
            false
        }
    } else {
        true
    }
}

fn parse_command(command : Option<&str>, command_param: Option<&str>, simp : &mut Tamagotchi) -> bool {
    if let Some(command)  = command {
        match command {
            "feed" => simp.feed(),
            "talk" => simp.talk(),
            "sleep" => {
                return parse_sleep_command(command_param ,simp);
            },
            _ => {
                return false;
            }
        }
    } else {
        return false; 
    }
    true
}

fn main() {
    let mut simp = Tamagotchi::create(String::from("Simp"));
    let mut line = String::new();
    println!(
        "Your goal is to keep your tamagotchi alive and happy \n
        In each command time moves 1 tick and stats reduced by 10
        If any of the stats goes zero or below you tamagotchi dies
        Here are the possible commands : \n
        talk => talk with tamagotchi increases social stat \n
        feed => feeds the tamagotchi increases fed stat \n
        sleep hour => make tamagotchi sleep increases sleep stat time moves 1 tick for every 4 hour \n
        Good Luck !
    ");
    while simp.alive {
        io::stdin().read_line(&mut line).expect("Failed to get the command");
        let mut command_iter = line.split_whitespace();
        let command = command_iter.next();
        let command_param = command_iter.next();
        let command_success = parse_command(command, command_param, &mut simp);
        if command_success {
            simp.stats();
            simp.time_tick();
        }
        line.clear();
    }
    println!("I am so sorry but your tamagotchi died :(");
}
