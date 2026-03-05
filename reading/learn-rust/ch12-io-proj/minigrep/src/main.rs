use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::process; // Added BufRead trait

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {err}");
        process::exit(1);
    });

    if let Err(e) = print_grep(&config) {
        println!("Application error: {e}");
        process::exit(2);
    }
}

struct Config {
    query: String,
    path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args!");
        }

        let query = args[1].clone();
        let path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            path,
            ignore_case,
        })
    }
}

fn print_grep(config: &Config) -> Result<(), Box<dyn Error>> {
    let fileobj = File::open(&config.path).expect("Failed to open file");
    let reader = io::BufReader::new(fileobj);

    for line in reader.lines() {
        // unwrap the io::Result<String>
        //   - if made error, it returns Err() object.
        //   - Box<dyn Error> will be covered later.  So far, just accept it.
        //
        let line_content = line.expect("Failed to read line");

        // String.contains()  -- note in README.md
        if config.ignore_case {
            let query = config.query.to_lowercase();
            if line_content.to_lowercase().contains(&query) {
                println!("{}: {}", &config.path, line_content);
            }
        } else {
            if line_content.contains(&config.query) {
                println!("{}: {}", &config.path, line_content);
            }
        };
    }

    Ok(())
}
