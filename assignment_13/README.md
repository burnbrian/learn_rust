# Instructions
Use the Fibonacci reccurrence relation to calculate the **nth** Fibonacci number. The next number in the Fibonacci sequence is the sum of the two previous numbers. Example shown below:

```text
F(0) = 0, F(1) = 1
F(n) = F(n - 1) + F(n - 2)
```

Try using both `if` statements and `match` to create your Fibonacci function.

# Resources
1. Only read [this forum article](https://users.rust-lang.org/t/optimizing-fast-fibonacci-computation/56933/19) if you are interested in feeling slower than your Fibonacci calculator.

# Example Output
```terminal_session
@burnbrian ➜ /workspaces/learn_rust/assignment_13 (main) $ cargo run
   Compiling assignment_13 v0.1.0 (/workspaces/learn_rust/assignment_13)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/assignment_13`
The 0 Fibonacci number is 0.
The 0 Fibonacci number is 0.
The 1 Fibonacci number is 1.
The 1 Fibonacci number is 1.
The 2 Fibonacci number is 1.
The 2 Fibonacci number is 1.
The 3 Fibonacci number is 2.
The 3 Fibonacci number is 2.
The 4 Fibonacci number is 3.
The 4 Fibonacci number is 3.
The 5 Fibonacci number is 5.
The 5 Fibonacci number is 5.
...
The 35 Fibonacci number is 9227465.
The 35 Fibonacci number is 9227465.
The 36 Fibonacci number is 14930352.
The 36 Fibonacci number is 14930352.
The 37 Fibonacci number is 24157817.
The 37 Fibonacci number is 24157817.
The 38 Fibonacci number is 39088169.
The 38 Fibonacci number is 39088169.
The 39 Fibonacci number is 63245986.
The 39 Fibonacci number is 63245986.
The 40 Fibonacci number is 102334155.
The 40 Fibonacci number is 102334155.
```