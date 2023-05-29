# Instructions
Prompt a user to input a radius. Calculate a circle's area given the user's radius with $`A = \pi r^2`$. Print the area to standard output.

# Bonus Points
1. Use `expect` to handle errors if user does not provide input
2. Use `expect` to handle errors if the user provides something strange like an emoji
3. Define PI as a constant before using it to perform the calculation

# Hints
1. You'll need to cast the radius variable and remove newlines `\n`
2. Shadow the user input to a number `f64`

# Example Output
```terminal_session
@burnbrian ➜ /workspaces/learn_rust/assignment_04 (main) $ cargo run
   Compiling assignment_04 v0.1.0 (/workspaces/learn_rust/assignment_04)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/assignment_04`
Enter a radius: 4.29
The area of a circle with radius 4.29 is 57.818136519.
```