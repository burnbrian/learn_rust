use std::io;
use std::io::Write;

fn main() {

    // create mutable variable for name
    let mut name = String::new();
    
    // ask user for name with input on same line
    print!("Please enter your name: ");
    io::stdout().flush().unwrap();

    // read user input into fname with expect error handling
    io::stdin()
        .read_line(&mut name)
        .expect("Did not read a first name, try again.");

    // print welcome message
    println!("Welcome {}", name);
}