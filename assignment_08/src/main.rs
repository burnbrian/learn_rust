use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    // initialize secret number
    let secret = rand::thread_rng().gen_range(1..=100);

    // print secret number for testing
    // println!("The secret number is: {}", secret);

    loop {

        // initialize guess
        let mut guess = String::new();

        // prompt user for a guess
        print!("Guess a number between 1 and 100: ");
            io::stdout().flush().unwrap();
        
        // read user input for guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // check input and convert to unsigned 32 bit integer
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        // check if guess is equal to secret
        match guess.cmp(&secret) {
            Ordering::Less => println!("{} is too low!", guess),
            Ordering::Greater => println!("{} is too high!", guess),
            Ordering::Equal => {
                println!("{} is correct!", guess);
                break;
            }
        }       
    }
}