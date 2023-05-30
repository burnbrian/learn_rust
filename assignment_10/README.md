# Instructions
Accept a user's hacker name as a command line argument then welcome the user via standard output.

# Bonus Points
1. Check how many arguments the user provided and return a help message if they entered more than one

# Resources
1. Checkout [The Rust Programming Langauge's](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html) article on accepting command line arguments
2. [Rust By Example's](https://doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html) article about argument parsing

# Example Output
```terminal_session
@burnbrian ➜ /workspaces/learn_rust/assignment_10 (main) $ cargo run -- derp derp derp
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/assignment_10 derp derp derp`
Usage: ./assignment_10 <hacker_name>

@burnbrian ➜ /workspaces/learn_rust/assignment_10 (main) $ cargo run -- derp derp
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/assignment_10 derp derp`
Usage: ./assignment_10 <hacker_name>

@burnbrian ➜ /workspaces/learn_rust/assignment_10 (main) $ cargo run -- terp
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/assignment_10 terp`
Welcome terp!
```