use std::env;
use std::io;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    print_grep(&config)
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
    let reader = io::BufReader::new(&config.path);
    for line in reader.lines() {
        let l = line?;
        println!("{}", l);
    }
}