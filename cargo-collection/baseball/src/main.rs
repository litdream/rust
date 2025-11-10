// Declare the module, now named guess_lib
mod guess_lib;

use std::io;

fn main() {
    println!("--- Number Guessing Game Setup ---");

    let com_num = guess_lib::generate_unique_digits();
    let mut userinput = String::new();

    while ! guess_lib::is_valid_guess(userinput.trim()) {
        userinput.clear();
        println!("Please enter a 3-digit number:");
        io::stdin()
            .read_line(&mut userinput)
            .expect("Failed to read line");
    }

    println!("Nice: {}", userinput.trim());
    println!("Generated number: {:?}", com_num);
}