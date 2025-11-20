// Gemini generated this code.
//
//   AND, it fails to compile.
//


use clap::Parser;  // cargo add clap


/// This application accepts a required positional argument, an optional output
/// file flag, and a boolean verbose flag.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The required input value, typically a filename or a mandatory string.
    #[arg(index = 1)]
    input_file: String,

    /// Optional flag to set the output file name.
    #[arg(short, long, value_name = "FILE")]
    output: Option<String>,

    /// Turn on verbose output (will print extra debugging information).
    #[arg(short, long)]
    verbose: bool,

    /// The processing mode, which can be 'fast' or 'safe'.
    #[arg(value_enum, default_value_t = Mode::Fast)]
    mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum Mode {
    /// Fast processing mode, less accurate.
    Fast,
    /// Safe processing mode, slower but more accurate.
    Safe,
}

fn main() {
    // This function automatically parses the arguments from the environment
    // and stores them in the Cli struct, or exits the program with an error
    // message if parsing fails (e.g., required arguments are missing).
    let cli = Cli::parse();

    println!("--- Arguments Parsed with CLAP ---");

    // 1. Handle Positional Argument
    println!("Input File: {}", cli.input_file);

    // 2. Handle Optional Flag/Switch (-o or --output)
    match cli.output {
        Some(filename) => println!("Output file specified: {}", filename),
        None => println!("No output file specified. Using default behavior."),
    }

    // 3. Handle Boolean Flag (-v or --verbose)
    if cli.verbose {
        println!("Verbose Mode: ON (Showing extra details)");
    } else {
        println!("Verbose Mode: OFF");
    }

    // 4. Handle Enum/Choice Argument
    match cli.mode {
        Mode::Fast => println!("Mode selected: Fast (default)"),
        Mode::Safe => println!("Mode selected: Safe"),
    }

    // Example of execution logic based on flags
    println!("\n--- Execution Preview ---");
    if cli.mode == Mode::Safe && cli.output.is_some() {
        println!("INFO: Starting SAFE processing and saving results.");
    } else {
        println!("INFO: Starting standard processing...");
    }
}
