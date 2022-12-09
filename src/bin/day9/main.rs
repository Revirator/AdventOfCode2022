use advent_of_code::reader;
use std::collections::HashSet;

fn main() {
    if let Ok(lines) = reader::read_input_lines("./src/bin/day9/input.txt") {
        // part 1
        let mut rope_1: [(i32, i32); 2] = [(0, 0); 2];
        let mut visited_positions_1: HashSet<(i32, i32)> = HashSet::new();
        visited_positions_1.insert((0, 0));
        // part 2
        let mut rope_2: [(i32, i32); 10] = [(0, 0); 10];
        let mut visited_positions_2: HashSet<(i32, i32)> = HashSet::new();
        visited_positions_2.insert((0, 0));
        for line in lines {
            if let Ok(ip) = line {
                move_head(&ip, &mut rope_1, &mut visited_positions_1);
                move_head(&ip, &mut rope_2, &mut visited_positions_2);
            }
        }
        println!("{}\n{}", visited_positions_1.len(), visited_positions_2.len());
    }
}

fn move_head(ip: &String, rope: &mut [(i32, i32)], visited_positions: &mut HashSet<(i32, i32)>) {
    let direction = &ip[0..1];
    let n = ip[2..].parse::<i32>().unwrap();
    match direction {
        "R" => {
            for _ in 0..n {
                rope[0].0 += 1;
                connect_points(rope, visited_positions);
            }
        }
        "L" => {
            for _ in 0..n {
                rope[0].0 -= 1;
                connect_points(rope, visited_positions);
            }
        }
        "U" => {
            for _ in 0..n {
                rope[0].1 += 1;
                connect_points(rope, visited_positions);
            }
        }
        "D" => {
            for _ in 0..n {
                rope[0].1 -= 1;
                connect_points(rope, visited_positions);
            }
        }
        _ => panic!("Invalid input")
    }
}

fn connect_points(rope: &mut [(i32, i32)], visited_positions: &mut HashSet<(i32, i32)>) {
    for i in 1..rope.len() {
        let head = rope[i - 1].clone();
        let tail = &mut rope[i];
        if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
            increment(head.0, &mut tail.0);
            increment(head.1, &mut tail.1);
        }
    }
    visited_positions.insert(rope.last().unwrap().clone());
}

fn increment(x: i32, y: &mut i32) {
    if x > *y {
        *y += 1;
    }
    if x < *y {
        *y -= 1;
    }
    // if equal: do nothing
}
