# Instructions
Ask a user for a number from 1 to 300. Display what range the number is from the following:

* 1-100
* 101-200
* 201-300

# Bonus Points
1. Check edge cases to verify your logic, e.g. 201, 0, -1

# Resources
1. [The Rust Reference's](https://doc.rust-lang.org/reference/expressions/if-expr.html) documentation on `if expressions`.
2. [The Rust Programming Language's](https://doc.rust-lang.org/book/appendix-02-operators.html) appendix of operators and symbols.

# Example Output
```terminal_session
@burnbrian ➜ /workspaces/learn_rust/assignment_07 (main) $ cargo run
   Compiling assignment_07 v0.1.0 (/workspaces/learn_rust/assignment_07)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/assignment_07`
Enter a number between 1 and 300: 50
The number 50 is between 1 and 100.

@burnbrian ➜ /workspaces/learn_rust/assignment_07 (main) $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/assignment_07`
Enter a number between 1 and 300: 150
The number 150 is between 100 and 200.

@burnbrian ➜ /workspaces/learn_rust/assignment_07 (main) $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/assignment_07`
Enter a number between 1 and 300: 250
The number 250 is between 200 and 300.

@burnbrian ➜ /workspaces/learn_rust/assignment_07 (main) $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/assignment_07`
Enter a number between 1 and 300: 350
The number 350 is outside of our range.
```