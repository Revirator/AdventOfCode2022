use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read, Result};

pub fn read_input_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)
        .expect(format!("Could not read file {}", filename).as_str());
    Ok(BufReader::new(file).lines())
}

pub fn read_input_to_chars(filename: &str) -> Vec<char> {
    let mut file = File::open(filename)
        .expect(format!("Could not read file {}", filename).as_str());
    let mut data = String::new();
    let _ = file.read_to_string(&mut data);
    data.chars().collect()
}

pub fn read_input_to_string(filename: &str) -> String {
    let mut file = File::open(filename)
        .expect(format!("Could not read file {}", filename).as_str());
    let mut data = String::new();
    let _ = file.read_to_string(&mut data);
    data
}