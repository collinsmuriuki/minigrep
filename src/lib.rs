#![allow(unused)]
use std::{fs,error::Error};
#[derive(Debug)]
pub struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'a str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn it_should_instantiate() {
        let test_vector: Vec<String> = Vec::new();
        Config::new(&test_vector).unwrap();
    }
}