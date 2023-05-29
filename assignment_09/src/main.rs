use std::io;
use std::io::Write;

fn main() {

    // check max size of int types
    println!("[!] Print max size of number types");
    println!("u8 max: {}", u8::max_value());
    println!("u32 max: {}", u32::max_value());
    println!("u128 max: {}", u128::max_value());

    // initialize number for checking
    let mut number = String::new();

    // prompt user for a number
    print!("Enter a number: ");
        io::stdout().flush().unwrap();
    
    // read user input for number
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line!");

    // check input and convert to unsigned 32 bit integer
    let number: u128 = match number.trim().parse() {
        Ok(n) => n,
        Err(_) => panic!("Invalid input!"),
    };

    // function to check if number is prime
    // check if number is prime
    if is_prime(number) {
        println!("The number {} is prime!", number);
    } else {
        println!("The number {} is not prime!", number);
    }
}

// function to check if number is prime
fn is_prime(number: u128) -> bool {
    if number <= 1 {
        return false;
    }
    // is number divisible by anything between 2 and number
    // prime ends in 1, 3, 7, or 9
    // could pre-check if number is even to save cycles?
    for a in 2..number {
        // modulo operator check for remainder
        if number % a == 0 {
            return false;
        }
    }
    true
}