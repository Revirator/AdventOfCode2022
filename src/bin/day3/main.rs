use advent_of_code::reader;
use std::collections::HashSet;

fn main() {
    let solution_1 = solve_1().unwrap();
    let solution_2 = solve_2().unwrap();
    println!("{solution_1}\n{solution_2}")
}

fn solve_1() -> Option<i32> {
    if let Ok(lines) = reader::read_input_lines("./src/bin/day3/input.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(ip) = line {
                let compartment_1 = &ip[..ip.len() / 2];
                let compartment_2 = &ip[ip.len() / 2..];
                sum += get_priority(intersect_two_strings(compartment_1, compartment_2));
            }
        }
        return Some(sum);
    }
    return None;
}

fn solve_2() -> Option<i32> {
    if let Ok(mut lines) = reader::read_input_lines("./src/bin/day3/input.txt") {
        let mut flag = lines.next();
        let mut sum = 0;
        while flag.is_some() {
            let line_1 = flag?.ok()?;
            let line_2 = lines.next()?.ok()?;
            let line_3 = lines.next()?.ok()?;
            sum += get_priority(intersect_three_strings(line_1.as_str(), line_2.as_str(), line_3.as_str()));
            flag = lines.next();
        }
        return Some(sum);
    }
    return None;
}

fn get_priority(i: u8) -> i32 {
    return if i <= 90 { i as i32 - 38 } else { i as i32 - 96 };
}

fn intersect_two_strings(s1: &str, s2: &str) -> u8 {
    let b1 = s1.bytes();
    let b2: HashSet<u8> = s2.bytes().collect();
    for b in b1 {
        if b2.contains(&b) {
            return b;
        }
    }
    panic!("Invalid input - {s1} and {s2} have no common character");
}

fn intersect_three_strings(s1: &str, s2: &str, s3: &str) -> u8 {
    let b1 = s1.bytes();
    let b2: HashSet<u8> = s2.bytes().collect();
    let b3: HashSet<u8> = s3.bytes().collect();
    for b in b1 {
        if b2.contains(&b) && b3.contains(&b) {
            return b;
        }
    }
    panic!("Invalid input - {s1}, {s2}, and {s3} have no common character");
}