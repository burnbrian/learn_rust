use std::io;
use std::io::Write;

fn main() {

    // get seconds from user
    print!("Please enter the number of seconds: ");
    io::stdout().flush().unwrap();

    // inititate variable for seconds
    let mut seconds = String::new();

    // read user input
    io::stdin()
        .read_line(&mut seconds)
        .expect("Failed to read line");

    // convert seconds to number
    let seconds: u32 = seconds.trim().parse()
        .expect("Please type a number!");

    // calculate hours, minutes, and seconds
    // `%` is the modulo operator
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let r_seconds = seconds - (hours * 3600) - (minutes * 60);

    // print results
    println!("{} seconds is {} hours, {} minutes, and {} seconds.",
             seconds, hours, minutes, r_seconds);
}
