# Credit
This challenge comes from *The Rust Programming Language*. The authors provide a great explanation of the code block on [their website](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).

# Instructions
Would you like to play a game? Randomly generate a secret number between 1 and 100. Prompt a user to provide a guess. Return less than or greater than until the user guesses the correct secret number.

# Bonus Points
1. Provide error handling if the user provides something other than a number, e.g. `qwerty`

# Hints
1. Add rand with `cargo add rand` or add `rand = "0.8.5"` to your `Cargo.toml` file
2. Use `use std::cmp::Ordering` to compare guesses
3. Use `use rand::Rng` to create the secret

# Resources
1. Read about `Rng` on [docs.rs](https://docs.rs/rand/0.8.5/rand/trait.Rng.html)
2. Greater than or less than in a match block discussion on the [Rust forums](https://users.rust-lang.org/t/greater-than-less-than-in-a-match-block/63399)
3. The Rust Programming Language [Operators and Symbols reference](https://doc.rust-lang.org/book/appendix-02-operators.html)

# Example Output
```terminal_session
@burnbrian ➜ /workspaces/learn_rust/assignment_08 (main) $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/assignment_08`
Guess a number between 1 and 100: 42
42 is too high!
Guess a number between 1 and 100: 21
21 is too high!
Guess a number between 1 and 100: 10
10 is too low!
Guess a number between 1 and 100: 15
15 is too high!
Guess a number between 1 and 100: 12
12 is correct!
```