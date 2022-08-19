use std::io;

fn fib(x: u32) -> u32 {
    if x > 2 {
        fib(x - 1) + fib(x - 2)
    } else if x == 2 {
        1
    } else {
        0
    }
}

fn main() {
    println!("Enter the a number 0 <= number <= 20");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read the line");

    let number: u32 = number.trim().parse().expect("Please enter a number");
    if number > 20 {
        println!("Number is greater than 20 exiting");
        return;
    }
    println!("The {number}th fibonacci number is {}", fib(number));
}
