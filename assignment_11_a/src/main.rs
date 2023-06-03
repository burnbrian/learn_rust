use std::io;
use std::io::Write;

// conversion of fahrenheit to celsius
// f = c * 9 / 5 + 32
// c = (f - 32) * 5 / 9

fn f_to_c(temp:f32) -> f32 {
    (temp - 32.0) * 5.0 / 9.0
}

fn c_to_f(temp:f32) -> f32 {
    (temp / 5.0 * 9.0) + 32.0
}

fn main() {
    // allow user to exit application
    println!(">>> Enter \"q\" or \"quit\" to exit the application...");

    loop {
       // get user input
        println!(">>> Enter temperature in the format \"32F\" or \"32C\"");
        print!(">>> Temperature: ");
            io::stdout().flush().unwrap();
        
        // read user input
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // allow user to exit before converting temperature
        if input.trim() == "q" || input.trim() == "quit" {
            println!(">>> Exiting temperature converter");
            break;
        }
        
        // trim input and split into temp and scale
        let (temp, scale) = input.trim().split_at(input.len() - 2);
        // convert temp to f32
        // println!("[!] DEBUG ----- temp: {}, scale: {}", temp, scale);
        let temp: f32 = match temp.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // convert the temps and print the results
        match scale.trim() {
            "c" => println!(">>> Result: {}C is {}F", temp, c_to_f(temp)),
            "C" => println!(">>> Result: {}{} is {}F", temp, scale, c_to_f(temp)),
            "f" => println!(">>> Result: {}F is {}C", temp, f_to_c(temp)),
            "F" => println!(">>> Result: {}{} is {}C", temp, scale, f_to_c(temp)),
            _ => continue,
        }
    }
}