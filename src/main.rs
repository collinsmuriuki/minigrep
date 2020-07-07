use std::{env,process};

use minigrep::{Config,run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem in parsing arguments, {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        println!("Application error, {}", err);
        process::exit(1);
    };
}
