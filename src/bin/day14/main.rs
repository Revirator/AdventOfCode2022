use std::cmp::{max, min};
use std::collections::HashSet;
use advent_of_code::reader;

fn main() {
    if let Ok(lines) = reader::read_input_lines("./src/bin/day14/input.txt") {
        let mut rocks: HashSet<(i16, i16)> = HashSet::new();
        let mut max_y = i16::MIN;
        for line in lines {
            if let Ok(ip) = line {
                let points = ip.split(" -> ").collect::<Vec<&str>>();
                for (i, p) in points.iter().enumerate() {
                    let p_start = p.split(',').collect::<Vec<&str>>();
                    let p_end = points.get(i + 1).unwrap_or(p).split(',').collect::<Vec<&str>>();
                    let p_start_x = p_start.get(0).unwrap().parse::<i16>().unwrap();
                    let p_end_x = p_start.get(1).unwrap().parse::<i16>().unwrap();
                    let p_start_y = p_end.get(0).unwrap().parse::<i16>().unwrap();
                    let p_end_y = p_end.get(1).unwrap().parse::<i16>().unwrap();
                    for j in min(p_start_x, p_start_y)..max(p_start_x, p_start_y) + 1 {
                        rocks.insert((j, p_end_x));
                    }
                    for j in min(p_end_x, p_end_y)..max(p_end_x, p_end_y) + 1 {
                        rocks.insert((p_start_x, j));
                    }
                    max_y = max(max(p_end_x, p_end_y), max_y);
                }
            }
        }
        //part 1
        drop_sand(&mut rocks.clone(), max_y, false);
        // part 2
        drop_sand(&mut rocks.clone(), max_y + 2, true);
    }
}

fn drop_sand(rocks: &mut HashSet<(i16, i16)>, max_y: i16, flag: bool) {
    let mut sand_units = 0_i16;
    'sand_cycles: loop {
        let mut position = (500, 0);
        loop {
            if rocks.contains(&position) && position.1 == 0 || position.1 > max_y {
                break 'sand_cycles;
            }
            if flag && position.1 == max_y - 1 {
                rocks.insert(position);
                sand_units += 1;
                break;
            }
            if rocks.contains(&(position.0, position.1 + 1)) {
                // try down left
                if !rocks.contains(&(position.0 - 1, position.1 + 1)) {
                    position.0 -= 1;
                    position.1 += 1;
                    continue;
                }
                // try down right
                if !rocks.contains(&(position.0 + 1, position.1 + 1)) {
                    position.0 += 1;
                    position.1 += 1;
                    continue;
                }
                rocks.insert(position);
                sand_units += 1;
                break;
            }
            position.1 += 1;
        }
    }
    println!("{}", sand_units);
}