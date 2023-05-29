# Instructions
Prompt the user for a number. Determine if the number is prime and return the primality to standard output.

# Bonus Points
1. Check multiple numbers until user quits the program
2. Create a separate function outside of main to check primality
3. Find out what the largest number you can store is, e.g. u32, u128

# Resources
1. Checkout this discussion on [StackOverflow](https://stackoverflow.com/questions/55790537/calculating-prime-numbers-in-rust) about checking primality without external crates

# Example Output
```terminal_session
@burnbrian ➜ /workspaces/learn_rust/assignment_09 (main) $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/assignment_09`
[!] Print max size of number types
u8 max: 255
u32 max: 4294967295
u128 max: 340282366920938463463374607431768211455
Enter a number: 17
The number 17 is prime!
```