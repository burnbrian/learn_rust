# Instructions
Use the [ferris-says](https://crates.io/crates/ferris-says) library to print a welcome message to standard output.

# Bonus Points
1. Set a custom width for your message
2. Read about the benefits of `std::io::BufWriter` [here](https://doc.rust-lang.org/std/io/struct.BufWriter.html)
3. Browse other crates at [Crates.io](https://crates.io)

# Hints
1. There are a few ways to import crates, try `cargo add <package>`
2. Configure the library in `Cargo.toml`

# Example Output
```terminal_session
@burnbrian ➜ /workspaces/learn_rust/assignment_02 (main) $ cargo run
Compiling assignment_02 v0.1.0 (/workspaces/learn_rust/assignment_02)
Finished dev [unoptimized + debuginfo] target(s) in 0.33s
Running `target/debug/assignment_02`
 _________________________
< What does the crab say? >
 -------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
```