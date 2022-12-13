use serde_json::{json, Value};
use serde_json::Value::{Array, Number};
use advent_of_code::reader;

fn main() {
    let input = reader::read_input_to_string("./src/bin/day13/input.txt");
    let mut lines = input.split("\r\n").collect::<Vec<&str>>();
    // part 1
    let chunks = lines.chunks(3);
    let mut sum = 0;
    for (i, chunk) in chunks.enumerate() {
        let left: Value = serde_json::from_str(chunk[0]).unwrap();
        let right: Value = serde_json::from_str(chunk[1]).unwrap();
        if let Some(true) = compare(&left, &right) {
            sum += i + 1;
        }
    }
    // part 2
    lines = lines.into_iter()
        .filter(|l| l.starts_with('['))
        .collect::<Vec<&str>>();
    lines.push("[[2]]");
    lines.push("[[6]]");
    sort(&mut lines);
    let packet_2 = lines.iter().position(|l| *l == "[[2]]").unwrap();
    let packet_6 = lines.iter().position(|l| *l == "[[6]]").unwrap();
    let distress_signal = (packet_2 + 1) * (packet_6 + 1);
    println!("{}\n{}", sum, distress_signal);
}

fn compare(left: &Value, right: &Value) -> Option<bool> {
    match left {
        Number(ln) => match right {
            Number(rn) => {
                if ln.as_u64() < rn.as_u64() {
                    return Some(true);
                }
                if ln.as_u64() > rn.as_u64() {
                    return Some(false);
                }
                None
            }
            Array(_) => compare(&json!([left]), right),
            _ => panic!("Invalid input")
        }
        Array(l) => match right {
            Number(_) => compare(left, &json!([right])),
            Array(r) => {
                for i in 0..l.len() {
                    // if right list ran out of items
                    if i == r.len() {
                        return Some(false);
                    }
                    let flag = compare(l.get(i)?, r.get(i)?);
                    if flag.is_some() {
                        return flag;
                    }
                }
                // if left list ran out of items
                if l.len() < r.len() {
                    return Some(true);
                }
                // else the lists are the same
                None
            }
            _ => panic!("Invalid input")
        }
        _ => panic!("Invalid input")
    }
}

fn sort(lines: &mut Vec<&str>) {
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            let left: Value = serde_json::from_str(lines.get(i).unwrap()).unwrap();
            let right: Value = serde_json::from_str(lines.get(j).unwrap()).unwrap();
            if let Some(false) = compare(&left, &right) {
                let temp = lines.get(j).unwrap().clone();
                *lines.get_mut(j).unwrap() = lines.get(i).unwrap().clone();
                *lines.get_mut(i).unwrap() = temp;
            }
        }
    }
}
