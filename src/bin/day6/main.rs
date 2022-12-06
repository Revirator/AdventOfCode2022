use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let input = read_input("./src/bin/day6/input.txt");
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

fn read_input(filename: &str) -> Vec<char> {
    let mut file = File::open(filename)
        .expect(format!("Could not read file {}", filename).as_str());
    let mut data = String::new();
    let _ = file.read_to_string(&mut data);
    data.chars().collect()
}