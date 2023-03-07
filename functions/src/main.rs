fn main() {
    hello_world("Hello, World!");
    statements();
    println!("The sum is equal to: {}", sum(5, 5))
}

fn hello_world(message: &str) {
    println!("{message}");
}

// Functions with Return Values
fn sum(x: u8, y: u8) -> u8 {
    x + y
}

// Statements and Expressions
fn statements() {
    let x = {
        let y = 3;
        y + 1
    };
    println!("The value of x is: {x}");
}
