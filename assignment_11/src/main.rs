use std::io;
use std::io::Write;

fn main() {
    // prompt the user for a Celsius temperature
    print!("What is the temperature in celsius: ");
    io::stdout().flush().unwrap();

    // define celsius
    let mut input_temp = String::new();
    
    // read the value in celsius
    io::stdin()
        .read_line(&mut input_temp)
        .expect("Failed to read line");

    // trim input, convert to f64
    let celsius:f64 = input_temp.trim().parse().unwrap();

    // print result of the conversion
    println!("{} degrees celsius is {} degrees Fahrenheit.", 
            celsius, celsius_to_fahrenheit(celsius));
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    // convert celsius to fahrenheit
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    fahrenheit
}