use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_inputs<P: AsRef<Path>>(filename: P) -> Vec<String> {
    let file = File::open(filename).expect("Failed to read file");
    return BufReader::new(file).lines().map(|line| line.unwrap()).collect();
}
