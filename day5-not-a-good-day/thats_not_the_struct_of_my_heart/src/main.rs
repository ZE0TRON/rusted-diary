#[derive(Debug)] // Adds the Debug trait, we need for print with {:?}
struct Friend {
    coward: bool,
    rude: bool,
    name: String,
    intelligent: u8
}

// Tuple Structs with out named fields
#[derive(Debug)]
struct Point(i32, i32, i32);

fn make_friend(name : String, intelligent: u8) -> Friend{
    // Smart naming since there is a variable with the same name as struct field we can omit it
    Friend { coward: false, rude: false, name, intelligent  }
}

fn clone_friend_with_same_attr(friend: &Friend, name:String) -> Friend{
    // Struct update syntax
    // Dereference friend to be able to extend it's fields
    Friend { name, ..*friend }
}

// Hello Friend, may be I should give you a name!
fn main() {
    let mut elliot = Friend {
        coward: true,
        rude: false,
        intelligent: 255,
        name: String::from("Elliot")
    };
    elliot.coward = false;
    let new_friend = make_friend(String::from("Darlene"), 255);
    let mr_robot = clone_friend_with_same_attr(&elliot, String::from("Mr. Robot"));
    println!("Bonsoir, {}", elliot.name);
    println!("Hello, {}", new_friend.name);
    println!("Hello, {}", mr_robot.name);
    let you_got_a_point = Point(0,0,0);
    println!("It makes {} sense", you_got_a_point.1);
    println!("{:?}", elliot);
   
    // Let's debug your mind
    // Dbg prints to the stderror with the line and file it's called
    // Also debug takes the ownership and returns it back, so make sure that
    // You pass the reference if you need the ownership later on
    dbg!(&mr_robot); 
    // Dbg also returns the evaluated value of the expression so can be used inline like this
    let new_point = Point(0, dbg!(you_got_a_point.1 + 5), 2);
    println!("{:?}", new_point);
}
