use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

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
