# Instructions
Prompt a user for a name and then print a welcome message to standard output.

# Bonus Points
1. Handle errors if the user does not provide input
2. Accept the user input on the same line as the prompt

# Hints
1. Check out `.expect` functionality
2. You may need another import, e.g. `use std::io::Write;`
3. This may help `io::stdout().flush().unwrap();`

# Example Output
```terminal_session
@burnbrian ➜ /workspaces/learn_rust/assignment_03 (main) $ cargo run
   Compiling assignment_03 v0.1.0 (/workspaces/learn_rust/assignment_03)
    Finished dev [unoptimized + debuginfo] target(s) in 0.66s
     Running `target/debug/assignment_03`
Please enter your name: Brian
Welcome Brian

```