use std::env;

fn main() {
    //.args() an iterator over the arguments passed in terminal
    //.collect() packs the input in vector
    let args : Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];
    println!("query: {}, filename: {}",query, filename);
    println!("args: {:?}",args);
}
