use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    // Parse arguments
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("Input file not provided")
    }

    // Read input file line by line
    let file = match fs::File::open(&args[1]) {
        Ok(v) => v,
        Err(e) => panic!("Could not open file."),
    };
    let reader = BufReader::new(file);

    // Read the file line by line.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        println!("{}. {}", index + 1, line);
    }
}