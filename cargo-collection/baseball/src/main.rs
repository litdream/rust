// Declare the module, now named guess_lib
mod guess_lib;

use std::io;

fn main() {
    println!("--- Number Guessing Game Setup ---");
    println!("The secret number is a 3-digit number with unique digits (1-9).");
    println!("Please enter your 3-digit guess now:");

    let mut guess = String::new();

    // Read input from the user
    io::stdin().read_line(&mut guess)
        .expect("Failed to read input line.");

    // Trim whitespace and the newline character
    let guess = guess.trim();

    // Check validity using the function from our module
    if guess_lib::is_valid_guess(guess) {
        println!("\n✅ Your guess '{guess}' is VALID and follows the rules. Great!");
        println!("(Now we'd check for hits and strikes... but that's the next step!)");
    } else {
        println!("\n❌ Your guess '{guess}' is INVALID.");
        println!("Reminder: Input must be 3 unique digits, each from 1-9.");
    }
}