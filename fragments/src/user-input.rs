use std::io;

fn main() {
    let mut userinput = String::new();

    println!("Please enter some text:"); // Prompt the user
    io::stdin()
        .read_line(&mut userinput)
        .expect("Failed to read line");

    let trimmed_input = userinput.trim();
    println!("You entered: -- {} --", trimmed_input);


    // Without clear, data is kept appended.

    println!("Please enter more of text:"); // Prompt the user
    io::stdin()
        .read_line(&mut userinput)
        .expect("Failed to read line");
    let tr2 = userinput.trim();
    println!("You entered: -- {} --", tr2);


    // So, we need clear
    userinput.clear();

    println!("Please enter new text, this time:"); // Prompt the user
    io::stdin()
        .read_line(&mut userinput)
        .expect("Failed to read line");
    let tr3 = userinput.trim();
    println!("You entered: -- {} --", tr3);
}
