use std::io;

fn main() {
    let mut userinput = String::new();

    println!("Please enter some text:"); // Prompt the user
    io::stdin()
        .read_line(&mut userinput)
        .expect("Failed to read line");

    let trimmed_input = userinput.trim();
    println!("You entered: {}", trimmed_input);
}
