use std::{env,fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Failed to read file");

    println!("With rext \n{}", contents);
}
struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Config {
        let query = &args[1];
        let filename = &args[2];

        Config { query, filename }
    }
}
