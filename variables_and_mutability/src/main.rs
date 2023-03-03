fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    // Declare a variable x and set its initial value to 5
    // Shadow the previous value of x by declaring a new variable x and set its value to the previous value of x plus 1
    // This is an example of shadowing, which allows us to reuse the same variable name and change its value while keeping the variable immutable
    let x: u32 = 5;
    let x: u32 = x + 1;
    println!("X value is: {x}");

    // Mutability
    // Declare a mutable variable y and set its initial value to 5
    // Assign a new value to the variable y by adding 1 to its current value
    // This is an example of mutability, which allows us to change the value of a variable
    let mut y = 5;
    y = y + 1;
    println!("Y value is: {y}");

    // Start a new scope and declare a new variable x
    // Set its value to the previous value of x times 2
    {
        let x: u32 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
