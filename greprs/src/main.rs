extern crate greprs;

use std::env;
use std::process;

use greprs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|
        {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);});

    println!("Seacrching for '{}'", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = greprs::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }

    // USE TO PRINT CURRENT DIRECTORY
    //let current_dir = env::current_dir().expect("unable to get current directory");
    //println!("Current directory: {:?}", current_dir);

}




