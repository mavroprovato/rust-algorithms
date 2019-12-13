use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use rust_algorithms::union_find::UnionFind;
use rust_algorithms::union_find::quick_find::UnionFindQuickFind;

fn main() {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Input file not provided")
    }

    // Open the input file
    let file = match File::open(&args[1]) {
        Ok(v) => v,
        Err(e) => panic!("Cannot open input file: {:?}", e),
    };
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Create the union find structure
    let size = match lines.next() {
        Some(v) => match v.unwrap().parse::<usize>() {
            Ok(v) => v,
            Err(e) => panic!("Cannot parse first line as an integer: {:?}", e),
        },
        None => panic!("File is empty"),
    };

    // Read the file line by line
    let mut uf = UnionFindQuickFind::new(size);
    for line in lines {
        let components: Vec<usize> = line.unwrap().split_whitespace().map(
            |s| {
                match s.parse::<usize>() {
                    Ok(v) => v,
                    Err(e) => panic!("Cannot parse component as an integer: {:?}", e),
                }
            }
        ).collect();
        if args.len() < 2 {
            panic!("Cannot parse line {}: need at least two integers")
        }
        uf.union(components[0], components[1])
    }

    println!("{:?}", uf);
}