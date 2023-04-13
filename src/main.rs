use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let input = line.expect("Failed to read line");
        
        match input.as_str() {
           "run 1" => println!("Run cmd 1: {}", input),
           "run 2" => println!("Run cmd 2: {}", input),
           "run 3" => println!("Run cmd 3: {}", input),
           "quit" => break,
           _ => println!("Unknown command {}", input),
        }
    }
}
