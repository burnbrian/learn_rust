// calculate Fibanocci numbers with ifs
fn fibonacci_if(f: u32) -> u32 {
    if f == 0 {
        return 0;
    } 
    else if f == 1 {
        return 1;
    }
    else {
        return fibonacci_if(f - 1) + fibonacci_if(f - 2);
    }
}

// calculate Fibonacci numbers with match
fn fibonacci_match(f: u32) -> u32 {
    match f {
        0 => 0,
        1 => 1,
        _ => fibonacci_match(f - 1) + fibonacci_match(f - 2),
    }
}

fn main() {
    for i in 0..41 {
        println!("The {} Fibonacci number is {}.", i, fibonacci_if(i));
        println!("The {} Fibonacci number is {}.", i, fibonacci_match(i));
    }
}
