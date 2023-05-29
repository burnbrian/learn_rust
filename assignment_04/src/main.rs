use std::io;
use std::io::Write;

fn main() {

    // declare constant PI
    const PI: f64 = 3.14159;

    // variable to store radius
    let mut radius = String::new();

    // prompt user for radius
    print!("Enter a radius: ");
    io::stdout().flush().unwrap();

    // read the input from the user
    io::stdin()
        .read_line(&mut radius)
        .expect("Failed to read line");

    // convert input to f64
    let radius: f64 = match radius.trim().parse() {
        Ok(num) => num,
        Err(_) => 42.0,
    };

    // calculate the area
    let area = PI * radius * radius;

    // print the radius and area
    println!("The area of a circle with radius {} is {}.", radius, area);
}