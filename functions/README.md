# Functions

This code provides a basic example of functions in Rust programming language.

## Function Definition

Functions in Rust are defined using the fn keyword, followed by the function name, parameter list, and return type (if
any).

```rust
fn hello_world(message: &str) {
    println!("{message}");
}
```

In this example, hello_world is a function that takes a single parameter of type &str and prints it to the console using
the println! macro.

## Function Call

To call a function in Rust, simply provide the function name followed by the arguments (if any) inside parentheses.

```rust
hello_world("Hello, World!");
```

## Functions with Return Values

Functions in Rust can also return values using the -> syntax to specify the return type.

```rust
fn sum(x: u8, y: u8) -> u8 {
    x + y
}
```

In this example, sum is a function that takes two parameters of type u8 and returns their sum as a u8 value.

## Statements and Expressions

Rust has two kinds of code constructs: statements and expressions. Statements are instructions that perform an action
but don't return a value, while expressions evaluate to a value.

```rust
fn statements() {
    let x = {
        let y = 3;
        y + 1
    };
    println!("The value of x is: {x}");
}
```

In this example, the code inside the curly braces is an expression that adds 1 to the value of y and returns the result.
This expression is assigned to the variable x.