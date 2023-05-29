use std::io;
use std::io::Write;

fn main () {

    // initialize variable
    let mut number = String::new();

    // prompt user for number
    print!("Enter a number between 1 and 300: ");
        io::stdout().flush().unwrap();

    // read user input for number
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line!");

    // check input and convert to unsigned 32 bit integer
    let number: i32 = number
        .trim()
        .parse()
        .expect("Please type a number!");

    // check if number is between 1 and 100
    if number >= 1 && number <= 100 {
        println!("The number {} is between 1 and 100.", number);
    }
    // greater than or equal to 101 and less than or equal to 200
    else if number >= 101 && number <= 200 {
        println!("The number {} is between 100 and 200.", number);
    }
    // greater than or equal to 201 and less than or equal to 300
    else if number >= 201 && number <= 300 {
        println!("The number {} is between 200 and 300.", number);
    }
    // catch all for numbers outside of our range
    else {
        println!("The number {} is outside of our range.", number);
    }
}