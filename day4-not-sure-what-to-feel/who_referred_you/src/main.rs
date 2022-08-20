fn main() {
    let mut s1 = String::from("I am real");
    // Below we pass the reference of s1 to the function, which is in simple words a pointer that points to the
    // actual s1, since reference's real value is just an address it's copied instead of moving,
    // References doesn't transfer the ownership 
    println!("s1 is {}", if is_non_empty_string(&s1) {"non empty"} else {"empty"}); // We pass the reference
    println!("s1's value is {s1}");
    s1.push_str(" are you ");
    println!("{s1}");
    // When using mutable references actually we are using the move semantics since having to mutable reference
    // to the same data can cause problems. But since we are still using the references ownership is not transfered.
    add_qm_to_string(&mut s1);
    println!("{s1}");
    {
        // No problem of having to non mutable refs to the same data at the same time
        let r1 = &s1;
        let r2 = &s1;
        println!("{r1}, {r2}");
    }
    {
        let r1 = &mut s1;
        // The lines below doesn't compile because you cannot have any other ref mutable or not to the 
        // same data when you have a mutable reference to that data, when it is dropped or goes out of scope
        // or not used in the first place then you are okay
        // let r2 = &mut s1;   
        // let r3 = &s1;
        // println!("{r1}, {r2}, {r3}");

        // The below lines compiles since we are not using r1 after declaring r2;
        // Note that a reference’s scope starts from where it is introduced and 
        // continues through the last time that reference is used. 
        println!("{r1}");
        let r2 = &mut s1;   
        println!("{r2}");
    }

    
}

fn is_non_empty_string(s : &String) -> bool { s.len() > 0 }

// fn add_dot_string(s : &String) { => Doesn't compile because if the reference is not mutable 
// you can't borrow a reference and change the value it points to even when the variable it self defined as mutable
// The mutability still referes to the mutability of the data in this case since we are not referencing another data

fn add_qm_to_string(s : &mut String) {
    s.push_str("?");
} 

// Doesn't compile because it creates a dangling reference
// fn dangle_that() -> &String {
//     let s = String::from("let me dangle!");
//     &s
// }