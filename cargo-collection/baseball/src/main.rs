// Declare the module, now named guess_lib
mod guess_lib;

use guess_lib::{compare_digits, generate_unique_digits, is_valid_guess, string_to_digit_vec};
use std::io;

fn main() {
    println!("--- Number Guessing Game Setup ---");

    let com_num = generate_unique_digits();
    let mut userinput = String::new();

    for round in 1..=10 {
        println!(" -- Round {round} --");

        while !is_valid_guess(userinput.trim()) {
            userinput.clear();
            println!("Please enter a 3-digit number:");
            io::stdin()
                .read_line(&mut userinput)
                .expect("Failed to read line");
        }
        let userinput_vec = string_to_digit_vec(userinput.trim());

        let cmp = compare_digits(&userinput_vec, &com_num);
        if cmp.0 == 3 {
            println!("CORRECT.  You found in {round} times.");
            return;
        }
        println!("{} strike, {} balls", cmp.0, cmp.1);
        userinput.clear();
    }

    println!("Sorry.  Generated number: {:?}", com_num);
}
