use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};

pub fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)
        .expect(format!("Could not read file {}", filename).as_str());
    Ok(BufReader::new(file).lines())
}