use std::{env,fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Failed to read file");

    println!("With rext \n{}", contents);
}
#[derive(Clone)]
struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(query: String, filename: String) -> Config {
        Config { query, filename }
    }
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config::new(query, filename)
}
