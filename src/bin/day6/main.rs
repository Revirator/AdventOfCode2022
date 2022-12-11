use advent_of_code::reader;
use std::collections::HashSet;

fn main() {
    let input = reader::read_input_to_chars("./src/bin/day6/input.txt");
    let mut m: Vec<char> = Vec::new();
    let n = 4; // for part 2 -> n = 14
    for i in 0..n {
        m.push(input.get(i).unwrap().clone())
    }
    for i in n..input.len() {
        if contains_unique_only(&m) {
            println!("{}", i);
            break;
        }
        m.remove(0);
        m.push(input.get(i).unwrap().clone());
    }
}

fn contains_unique_only(vec: &Vec<char>) -> bool {
    let mut unique: HashSet<char> = HashSet::new();
    vec.into_iter().all(|x| unique.insert(x.clone()))
}