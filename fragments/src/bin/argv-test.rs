use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(program_name) = args.get(0) {
        println!("Program Name (Argument 0): {}", program_name);
    }

    println!("Total arguments received: {}", args.len());

    for (index, arg) in args.iter().enumerate().skip(1) {
        println!("Argument {} (Index {}): {}", index, index, arg);
    }

    if let Some(second_arg) = args.get(2) {
        println!("\nAccessing the second user argument directly: args.get(2)");
        println!("The argument at index 2 is: {}", second_arg);
    } else {
        println!("\nFewer than two user arguments provided.");
    }
}
