use std::cmp::{max, min};
use std::collections::HashSet;
use regex::Regex;
use advent_of_code::reader;

fn main() {
    if let Ok(lines) = reader::read_input_lines("./src/bin/day15/input.txt") {
        let mut sensors: HashSet<Sensor> = HashSet::new();
        let mut beacons: HashSet<Beacon> = HashSet::new();
        for line in lines {
            if let Ok(ip) = line {
                let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
                let cap = re.captures(ip.as_str()).unwrap();
                let mut sensor = Sensor(cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap(), 0);
                let beacon = Beacon(cap[3].parse::<i32>().unwrap(), cap[4].parse::<i32>().unwrap());
                sensor.2 = manhattan(&sensor, &beacon);
                sensors.insert(sensor);
                beacons.insert(beacon);
            }
        }
        let y_coord = 2_000_000;
        let mut non_beacons_positions = beacons.clone();
        let mut non_beacons_count = 0;
        // part 1
        for sensor in &sensors {
            if (y_coord > sensor.1 && y_coord <= sensor.1 + sensor.2) ||
                (y_coord < sensor.1 && y_coord >= sensor.1 - sensor.2) {
                let x_coords = reverse_manhattan(sensor, y_coord);
                for i in x_coords.0..x_coords.1 + 1 {
                    let b = Beacon(i, y_coord);
                    if non_beacons_positions.insert(b) {
                        non_beacons_count += 1;
                    }
                }
            }
        }
        println!("{}", non_beacons_count);
        // part 2
        let mut lines_1: HashSet<i32> = HashSet::new();
        let mut lines_2: HashSet<i32> = HashSet::new();
        for sensor in &sensors {
            lines_1.insert(sensor.1 - sensor.0 + sensor.2 + 1);
            lines_1.insert(sensor.1 - sensor.0 - sensor.2 - 1);
            lines_2.insert(sensor.1 + sensor.0 + sensor.2 + 1);
            lines_2.insert(sensor.1 + sensor.0 - sensor.2 - 1);
        }
        'outer: for i in &lines_1 {
            for j in &lines_2 {
                // find the intersection point
                let x = (*j - *i) / 2;
                let y = (*j + *i) / 2;
                if x >= 0 && x <= 4_000_000 && y >= 0 && y <= 4_000_000 {
                    let b = Beacon(x, y);
                    let mut flag = true;
                    for sensor in &sensors {
                        if manhattan(sensor, &b) <= sensor.2 {
                            flag = false;
                            break;
                        }
                    }
                    if flag {
                        println!("{}", x as u64 * 4_000_000 + y as u64);
                        break 'outer;
                    }
                }
            }
        }
    }
}

fn manhattan(sensor: &Sensor, beacon: &Beacon) -> i32 {
    (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()
}

fn reverse_manhattan(sensor: &Sensor, y_coord: i32) -> (i32, i32) {
    let d = sensor.2 - (sensor.1 - y_coord).abs();
    let (x_1, x_2) = (sensor.0 - d, sensor.0 + d);
    (min(x_1, x_2), max(x_1, x_2))
}

#[derive(Eq, Hash, PartialEq, Clone)]
// (x, y, distance to closest beacon)
struct Sensor(i32, i32, i32);

#[derive(Eq, Hash, PartialEq, Clone)]
// (x, y)
struct Beacon(i32, i32);