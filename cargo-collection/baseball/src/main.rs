// Declare the module, now named guess_lib
mod guess_lib;

use std::io;
use guess_lib::{generate_unique_digits, is_valid_guess, string_to_digit_vec};

fn main() {
    println!("--- Number Guessing Game Setup ---");

    let com_num = generate_unique_digits();
    let mut userinput = String::new();

    while ! is_valid_guess(userinput.trim()) {
        userinput.clear();
        println!("Please enter a 3-digit number:");
        io::stdin()
            .read_line(&mut userinput)
            .expect("Failed to read line");
    }

    let userinput_vec = string_to_digit_vec(userinput.trim());

    println!("Nice: {:?}", userinput_vec );
    println!("Generated number: {:?}", com_num);
}