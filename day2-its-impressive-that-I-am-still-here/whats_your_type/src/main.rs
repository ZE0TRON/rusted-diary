fn increment(x: u8) -> u8 {
    return x + 1;
}

fn main() {
    // Infering vs explicit type
    let infer_me_till_inferno = 666;
    let infer_me_not: u32 = "42"
        .parse()
        .expect("May be it's not the answer to everything");
    // Different ways of representing const numbers;
    let separe_me_mi_lord: u32 = 1_000_000_000;
    let do_i_look_overflowed: u32 = 0xFFFFFFFF;
    let the_little_fella: u8 = 0b1111_000_1;
    let simple_fella: u8 = b'A';

    // Overflows
    let x: u8 = 255; 
    // Following line creates a panic in development mode, in release mode overflowed becomes 0
    // println!("x : {}, x+1(overflowed) : {} ", x , increment(x)); 
    // Overflow safe functions
    let x: u8 = u8::wrapping_add(255, 2 );
    println!("{x}"); // 1
    let x = match u8::checked_add(255, 2) {
        Some(x) => {x},
        None => {0},
    };
    println!("{x}"); // 0

    // Floats
    let floating_away = 3.9; // f64
    let floating : f32 = 3.9; // f32
    let let_me_down = 2 / 3; // 0    

    // Booleans
    let i_will_always_reveal = true;
    let rust_sucks : bool = false;

    // Chars
    // Chars are four bytes and unicode scalar values , WOW!
    // Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF
    // If you need just ascii use u8 when space is limited;
    let c = 't';
    let z : char = 'Ç';
    let my_mind = '🤯';


}
