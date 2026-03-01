use std::env;
use std::io::{self, BufRead}; // Added BufRead trait
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    print_grep(&config);
}

struct Config {
    query: String,
    path: String,
}

fn parse_config(args: &[String]) -> Config {
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let query = args[1].clone();
    let path = args[2].clone();
    Config { query, path }
}

fn print_grep(config: &Config) {
    let fileobj = File::open(&config.path).expect("Failed to open file");
    let reader = io::BufReader::new(fileobj);

    for line in reader.lines() {
        let line_content = line.expect("Failed to read line");
        if line_content.contains(&config.query) {
            println!("{}: {}", &config.path, line_content);
        }
    }
}
