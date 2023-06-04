#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // reference to struct
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // iterate through six rectangles
    for i in 0..6 {
        let rectangle = Rectangle {
            width: 50 + i,
            height: 60 + i,
        };
        // falls out of scope after referencing the struct area function
        println!("The area of a rectangle with width {} and height {} is {}.", 
            rectangle.width, rectangle.height, rectangle.area());
    }
}