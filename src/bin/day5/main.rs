use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Lines};

#[path = "../reader.rs"]
mod reader;

fn main() {
    if let Ok(mut lines) = reader::read_lines("./src/bin/day5/input.txt") {
        let mut stacks = setup_initial_state(&mut lines);
        lines.next();
        for line in lines {
            if let Ok(ip) = line {
                let l: Vec<&str> = ip.split(' ').collect();
                let number_of_crates = l[1].parse::<usize>().ok().unwrap();
                let old_stack = stacks.get_mut(l[3].parse::<usize>().ok().unwrap() - 1).unwrap();
                let mut temp: Vec<char> = Vec::new();
                for _i in 0..number_of_crates {
                    // part 1
                    temp.insert(0, old_stack.pop().unwrap());
                    // part 2
                    //temp.push(old_stack.pop().unwrap());
                }
                let new_stack = stacks.get_mut(l[5].parse::<usize>().ok().unwrap() - 1).unwrap();
                for _i in 0..number_of_crates {
                    new_stack.push(temp.pop().unwrap());
                }
            }
        }
        let mut result = String::from("");
        for c in stacks {
            result.push(c.last().unwrap().clone());
        }
        println!("{}", result)
    }
}

fn setup_initial_state(lines: &mut Lines<BufReader<File>>) -> [Vec<char>; 9] {
    let re = Regex::new(r"[\[ ]([A-Z ])[] ] [\[ ]([A-Z ])[] ] [\[ ]([A-Z ])[] ] [\[ ]([A-Z ])[] ] [\[ ]([A-Z ])[] ] [\[ ]([A-Z ])[] ] [\[ ]([A-Z ])[] ] [\[ ]([A-Z ])[] ] [\[ ]([A-Z ])[] ]").unwrap();
    let mut stacks: [Vec<char>; 9] = Default::default();
    for line in lines {
        if let Ok(ip) = line {
            if re.is_match(ip.as_str()) {
                let x = re.captures(ip.as_str()).unwrap();
                for i in 1..10 {
                    if &x[i] != " " {
                        let stack = stacks.get_mut(i - 1).unwrap();
                        stack.insert(0, *&x[i].parse::<char>().unwrap())
                    }
                }
            } else {
                break;
            }
        }
    }
    return stacks;
}