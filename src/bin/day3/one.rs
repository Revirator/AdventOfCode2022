use std::collections::HashSet;

#[path = "../reader.rs"]
mod reader;

pub fn solve() -> Option<i32> {
    if let Ok(lines) = reader::read_lines("./src/bin/day3/input.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(ip) = line {
                let compartment_1 = &ip[..ip.len() / 2];
                let compartment_2 = &ip[ip.len() / 2..];
                sum += get_priority(intersect(compartment_1, compartment_2));
            }
        }
        return Some(sum);
    }
    return None;
}

fn get_priority(i: u8) -> i32 {
    return if i <= 90 { i as i32 - 38 } else { i as i32 - 96 };
}

fn intersect(s1: &str, s2: &str) -> u8 {
    let b1 = s1.bytes();
    let b2: HashSet<u8> = s2.bytes().collect();
    for b in b1 {
        if b2.contains(&b) {
            return b;
        }
    }
    panic!("Invalid input - {s1} and {s2} have no common character");
}