use std::env;

fn main() {
    // use std::env::args to collect cli arguments
    let args: Vec<String> = env::args().collect();
    
    // userful for debugging args
    // dbg!(args);

    // set hacker_name to the second argument
    let hacker_name = &args[1];

    // check number of args and welcome the hacker
    match args.len() {
        1 => println!("Usage: ./assignment_10 <hacker_name>"),
        2 => println!("Welcome {}!", hacker_name),
        _ => println!("Usage: ./assignment_10 <hacker_name>")
    }
}