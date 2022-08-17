const FIRST_YEAR_OF_COVID: u32 = 2020;
const SECOND_YEAR_OF_COVID: u32 = FIRST_YEAR_OF_COVID + 1;

fn main() {
    let x = "Book";
    println!("This is the original unchanged {x}");
    let mut my_salary = 100; // No decrement, only increment please :)
    println!("My salary before the rise {my_salary}");
    my_salary += my_salary / 10;
    println!("My salary after the rise {my_salary}");
    println!("{FIRST_YEAR_OF_COVID} and {SECOND_YEAR_OF_COVID} were pretty awkward.");
    let mut x = "ORIGINAL BOOK";
    println!("This is the original unchanged {x}");
    x = "DEFINTELY ORIGINAL BOOK";
    println!("This is the original unchanged {x}");
    {
        let x = "SCOPed DOPE BOOK";
        println!("This is scope has it's own originals and respect the others with {x}");
    }
    println!("This is the original unchanged {x}"); // DEFINETELY ORIGINAL BOOK
    {
        let x = 420; // Not even a book zZzZz
        println!("Some say {x} is the book of nature");
    }
}
