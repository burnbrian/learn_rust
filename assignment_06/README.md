# Instructions
Prompt a user for a numerator and denominator. Perform the division and inform the user if there is a remainder or not.

# Bonus Points
1. If there is a remainder print it to standard output
2. Perform the math and return the quotient is applicable

# Hints
1. Checkout this page on [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-03-if-let.html) that details `if let` control flow

# Example Output
```terminal_session
@burnbrian ➜ /workspaces/learn_rust/assignment_06 (main) $ cargo run
   Compiling assignment_06 v0.1.0 (/workspaces/learn_rust/assignment_06)
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/assignment_06`
Please enter the numerator: 15
Please enter the denominator: 2
15 divided by 2 is 7 with a remainder of 1.

@burnbrian ➜ /workspaces/learn_rust/assignment_06 (main) $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/assignment_06`
Please enter the numerator: 14
Please enter the denominator: 7
14 divided by 7 is 2 with no remainder.

@burnbrian ➜ /workspaces/learn_rust/assignment_06 (main) $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/assignment_06`
Please enter the numerator: 6
Please enter the denominator: 7
6 cannot be divided by 7.
```