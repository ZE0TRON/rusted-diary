fn increment(x :u32) -> u32 { x + 1}

fn increment_non_zero(x: u32) -> u32 {
    if x == 0 {
        return 0;
    }
    x + 1
}

fn increment_non_negative(x : i32) -> i32 {
    if x >= 0 {
        x + 1
    } else {
        x
    }
}

fn increment_non_negative_non_zero (x : i32) -> i32 {
    if x == 0 {
        return 0;
    } else {
        increment_non_negative(x)
    }
}

fn increment_both(x : (u32, u32)) -> (u32, u32) {(increment(x.0), increment(x.1))}

fn main() {
    // Expressions
    // No semicolons on expressions only in statements;
    let expressionism = {
        let art = 4;
        let abstract_art = art + 1;
        abstract_art + 1
    };
    println!("Expressionism : {expressionism}");

    let meaningless = {
        let meaining = i64::MAX;
        meaining;
    };
    println!("Meaningless : {}", meaningless == ());
    println!("increment 5 by 1 = {}", increment(5));
    println!("Non zero :\n increment 5 by 1 = {}, increment zero by 1 = {}", increment_non_zero(5), increment_non_zero(0));
    println!("None Negative :\nincrement 5 by 1 = {}, increment -2 by 1 = {}", increment_non_negative(5), increment_non_negative(-2));
    let tuple = (2,3);
    println!("increment tuple {:?} by 1 = {:?}", tuple, increment_both(tuple));

}
