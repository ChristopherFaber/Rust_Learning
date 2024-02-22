use std::env;
use std::io;
use std::io::Write;
use std::env::args;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Please enter string: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim().to_string();
    println!("{:?}", input);

    let output:Vec<&str> = input.split(":").collect();
    println!("{:?}", output);
}
