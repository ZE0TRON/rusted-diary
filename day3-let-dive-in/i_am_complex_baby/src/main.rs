fn main() {

    // Tuples
    let _a_went_into_a_bar: (i32, f64, u8, char) = (231, 43.2, 4, '🍺');
    let lotr = ('👦', '💍', '🌋');
    let (frodo, ring, mount_doom) = lotr;
    let one_ring = lotr.1;
    /*
        The tuple without any values has a special name, unit. 
        This value and its corresponding type are both written () 
        and represent an empty value or an empty return type.
        Expressions implicitly return the unit value if they don’t return any other value. 
    */
    let _unit = ();

    // Arrays
    let boring_array = [1,2,3,4,5];
    let i_am_more_boring : [u32; 3] = [1,2,3];
    let may_be_interesting = [3.14 ; 3]; // [3.14, 3.14, 3.14] 
    let am_i_possible = [[1,2,3,4], [1,2,3,4]];
    let boring_access = boring_array[2];
    let meh_access = am_i_possible[1][2];
    let possible_arr = am_i_possible[1];

    // Note:!!! invalid array access(invalid index) because of run time value 
    // results in panic in Rust memory doesnt get accessed but 
    // rust makes the check on run time and panics.
}
