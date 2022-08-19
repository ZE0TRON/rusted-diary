fn main() {
    // what would happen if we had fun
    let num1 = 98;
    let num2 = 87;
    let give_me_max = if num1 > num2 { num1 } else { num2 };
    println!("max({}, {}) = {}", num1, num2, give_me_max);
    println!(
        "min({}, {}) = {}",
        num1,
        num2,
        if num1 < num2 { num1 } else { num2 }
    );

    // Loops
    let mut i_can_control = 0;

    // Returning values from loops using break weird ~
    let loop_result = loop {
        i_can_control += 1;
        if i_can_control > 5 {
            break i_can_control * 2;
        }
    };

    let mut i = 0;
    // Loop labels more weird shit
    // Used to disambiguate the breaks and continues when using nested loops
    'outside_loop: loop {
        loop {
            i += 1;
            if i > 10 {
                break 'outside_loop;
            } else if i > 5 {
                break;
            }
        }
    }

    // classic while
    while i < 20 {
        i += 1;
    }

    // collection looping with for
    let arr = [1, 2, 3];
    for elem in arr {
        print!("{elem}, ");
    }
    println!();
    // Ranges
    for number in (1..4).rev() { // 4..1 doesn't work return empty iterator or something like that
        print!("{number}, ");
    }
    println!();

}
