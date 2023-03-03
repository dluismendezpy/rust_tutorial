use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_game() {
    const MIN_SECRET_NUMBER: u32 = 1;
    const MAX_SECRET_NUMBER: u32 = 10;

    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(MIN_SECRET_NUMBER..=MAX_SECRET_NUMBER);

    loop {
        println!("\nPlease input your guess.");

        // Declare a mutable variable guess and set its value to an empty string
        let mut guess = String::new();

        // Read the user's input and store it in the guess variable
        // If an error occurs, print "Failed to read line" to the console
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the guess string to an unsigned 32-bit integer
        // If the conversion is successful, assign the result to guess variable
        // If not, continue with the next iteration of the loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Print "Too small!" if the guess is less than the secret number, "Too big!" if the guess is greater, and "YOU WIN!!" if the guess is equal to the secret number, then break out of the loop.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("\nYOU WIN!!");
                break;
            }
        }
    }
}
