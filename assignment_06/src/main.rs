use std::io;
use std::io::Write;

fn main() {

    // initiate variable for numerator and denominator
    let mut numerator = String::new();
    let mut denominator = String::new();

    // prompt user for numerator
    print!("Please enter the numerator: ");
    io::stdout().flush().unwrap();

    // read user input for numerator
    io::stdin()
        .read_line(&mut numerator)
        .expect("Failed to read line!");

    // check input and convert to unsigned 32 bit integer
    let numerator: u32 = numerator
        .trim()
        .parse()
        .expect("Please type a number!");

    // prompt user for denominator
    print!("Please enter the denominator: ");
    io::stdout().flush().unwrap();

    // read user input for denominator
    io::stdin()
        .read_line(&mut denominator)
        .expect("Failed to read line!");

    // check input and convert to unsigned 32 bit integer
    let denominator: u32 = denominator
        .trim()
        .parse()
        .expect("Please type a number!");

    // perform division and print remainder
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;

    // divisible with remainder
    if quotient > 0 && remainder > 0 {
        println!("{} divided by {} is {} with a remainder of {}.", 
                numerator, denominator, quotient, remainder);
    // divisible with no remainder
    } else if quotient > 0 && remainder == 0 {
        println!("{} divided by {} is {} with no remainder.", 
                numerator, denominator, quotient);
    // not divisible for our purposes
    } else {
        println!("{} cannot be divided by {}.", 
                numerator, denominator);
    };
}