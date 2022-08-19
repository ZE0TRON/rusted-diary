fn main() {
    let mut s = String::from("hi");
    s.push_str(&", people");
    println!("{s}");
    {
        let butterfly = String::from("I live short you know");
        println!("Butteryfly says : {butterfly}");
    } // drop gets called on butterfly since it goes out of scope eg, implicit call to drop(butterfly)
    // butterfly not reachable from here;

    // Auto moves and shallow copies
    let s1  = String::from("test");
    let s2 = s1; // s1 becomes invalid after this line to prevent double free
    // A move means like creating a shallow copy and invalidating the first variable;
    // In the example s1 is moved to s2

    // println!("{}", s1); => Doesnt compile because s1 is moved to s2 and cannot be borrowed

    let outer_str = String::from("outer");
    {
        let inner_str = outer_str; // outer_str invalidated
    } // scope ended for inner_str we no longer have access to the data with either outer or innter str variable 
    // println!("{outer_str}"); => Again doesn't compile because outer moved to inner then inner got out of scope

    // Deep Copies, clones
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2); // No erros because this time we have copied the value

    // Basic types are not moved directly copied, since they are inexpensive the copy
    // This types implement the Copy trait and stored directly on the stack instead of heap, eg. u32, char, tuples, arrays
    // You cant annotate a type with Copy if the type or any parts of it implemented the Drop trait
    // Example below shows arrays indeed uses copy so be aware, since this can cost some time!
    let arr = [12,3,4];
    {
        let mut arr2 = arr;
        arr2[0] = 4;
        println!("{:?}", arr2);
        println!("{:?}", arr);
    }
    println!("{:?}", arr);

    let s1 = gives_onwership();
    let s2 = String::from("given");
    takes_ownership(s2);
    // println!("{s2}"); => Doesn't compile because the function took the ownership and didnt give back
    // in other word value moved when passing to the function and after function scope drop called

    let s3 = takes_and_gives_back(s1);
    println!("{s3}") // since the ownership given back s3 = s1 and it's valid

}

fn takes_ownership(str : String) {
    println!("{str}");
}

fn gives_onwership() -> String {
   let str = String::from("here");
   str
}

fn takes_and_gives_back(str : String) -> String {
    str
}

