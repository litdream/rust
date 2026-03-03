use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process; // Added BufRead trait

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {err}");
        process::exit(1);
    });

    print_grep(&config);
}

struct Config {
    query: String,
    path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args!");
        }
        let query = args[1].clone();
        let path = args[2].clone();
        Ok(Config { query, path })
    }
}

fn print_grep(config: &Config) {
    let fileobj = File::open(&config.path).expect("Failed to open file");
    let reader = io::BufReader::new(fileobj);

    for line in reader.lines() {
        let line_content = line.expect("Failed to read line"); // unwrap the io::Result<String>

        // String.contains()  -- note in README.md
        if line_content.contains(&config.query) {
            println!("{}: {}", &config.path, line_content);
        }
    }
}
