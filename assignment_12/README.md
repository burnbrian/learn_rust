# Instructions
Use `impl`, `struct`, and `iter` to print the areas of rectangles with the following characteristics:

1. Width: `50`, Height: `60`
2. Width: `51`, Height: `61`
3. Width: `52`, Height: `62`
4. Width: `53`, Height: `63`
5. Width: `54`, Height: `64`
6. Width: `55`, Height: `65`

# Hints
1. Use `#[derive(Debug)]`,`{:?}`, and `dbg!(&rectangle)` to debug your `struct`

# Example Output
```terminal_session
@burnbrian ➜ /workspaces/learn_rust/assignment_12 (main) $ cargo run
   Compiling assignment_12 v0.1.0 (/workspaces/learn_rust/assignment_12)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/assignment_12`
The area of a rectangle with width 50 and height 60 is 3000.
The area of a rectangle with width 51 and height 61 is 3111.
The area of a rectangle with width 52 and height 62 is 3224.
The area of a rectangle with width 53 and height 63 is 3339.
The area of a rectangle with width 54 and height 64 is 3456.
The area of a rectangle with width 55 and height 65 is 3575.
```