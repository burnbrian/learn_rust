# Instructions
This is an iteration of the previous challenge. We're still converting Fahrenheit to Celsius, but this time with a few tweaks:

1. Loop the requests until the user exits
2. Convert from C->F and F->C based on the user's input

# Bonus Points
1. Match both uppercase and lowercase characters from user input, e.g. `C` and `c`
2. Implement `match` in your solution

# Resources
1. More advanced example using enums on [Benjaminbrandt.com](https://benjaminbrandt.com/converting-temperatures-in-rust/)
2. Checking user input from [this StackOverflow article](https://stackoverflow.com/questions/71862314/how-can-i-compare-a-user-input-to-a-specific-char) shows how to allow a user to exit the application loop
3. Confirm your calculations with [RapidTables](https://www.rapidtables.com/convert/temperature/celsius-to-fahrenheit.html)

# Example Output
```terminal_session
@burnbrian ➜ /workspaces/learn_rust/assignment_11_a (main) $ cargo run
   Compiling assignment_11_a v0.1.0 (/workspaces/learn_rust/assignment_11_a)
    Finished dev [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/assignment_11_a`
>>> Enter "q" or "quit" to exit the application...
>>> Enter temperature in the format "32F" or "32C"
>>> Temperature: 98.9F
>>> Result: 98.9F is 37.166668C
>>> Enter temperature in the format "32F" or "32C"
>>> Temperature: 42C
>>> Result: 42C is 107.6F
>>> Enter temperature in the format "32F" or "32C"
>>> Temperature: -22F
>>> Result: -22F is -30C
>>> Enter temperature in the format "32F" or "32C"
>>> Temperature: quit
>>> Exiting temperature converter
```