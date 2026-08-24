use core::error;
use std::env;
use std::fs;
use std::process;
use crate::error::Error;

fn main() {
    //.args() an iterator over the arguments passed in terminal
    //.collect() packs the input in vector
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("Error: {}", error);
        process::exit(1)
    });
    println!("query: {}, filename: {}", config.query, config.filename);
    println!("args: {:?}", args);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1)
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    println!("With text: \n{}", content);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
