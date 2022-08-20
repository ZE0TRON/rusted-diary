fn main() {
    let listening_now = String::from("Max Cooper");
    // Lets take slices
    let name = &listening_now[0..3];
    let surname = &listening_now[4..];
    println!("Listening now {} {}", name, surname);
    let playing_now = String::from("Max Cooper-Repetition");
    // Following lines doesn't compile and prevents a bug because we try to get a 
    // mutable reference while we already
    // have a immutable reference
    // We first get a immutable borrow song = get_song then we make a 
    // mutable borrow playing_now.clear() and later
    // we try to use the immutable borrow
    // let song = get_song(&playing_now);
    // playing_now.clear();
    // println!("Song : {}", song);
    println!("Playing now {}", get_artist(&playing_now));
    println!("Song : {}", get_song(&playing_now));
    // Suprise string literals are actually slices what!
    // They point to a specific point in compiled binary instead of memory
    // and that's why they are immutable
    let s: &str = "Suprise!";


    // You can also slice other stuff
    let arr = [1,3,5,7,9];
    let slice = &arr[1..4]; // &[i32] type
    assert_eq!(slice, &[3,5,7]);

}

// We can use the &str in function signature since a reference to a string is 
// also a full slice of the string
fn get_artist(play_data: &str) -> &str {
    let bytes = play_data.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b'-' {
            return &play_data[..index];
        }
    }

    &play_data[..]
}

fn get_song(play_data: &String) -> &str {
    // Type of string slice is &str
    let bytes = play_data.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b'-' {
            return &play_data[index + 1..];
        }
    }

    &play_data[..]
}
