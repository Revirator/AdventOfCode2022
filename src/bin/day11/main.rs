use eval::Expr;
use eval::Value::Number;
use advent_of_code::reader;

// set to false for part 1 and true for part 2
const IS_PART_2: bool = true;

fn main() {
    let input = reader::read_input_to_string("./src/bin/day11/input.txt");
    let lines = input.split("\r\n").collect::<Vec<&str>>();
    let chunks = lines.chunks(7);
    let mut monkeys: Vec<Monkey> = Vec::new();
    for chunk in chunks {
        monkeys.push(Monkey::parse(chunk));
    }
    let lcm = &monkeys.iter().fold(1, |acc, x| acc * x.1);
    let n = if IS_PART_2 { 10000 } else { 20 };
    for _ in 0..n {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get(i).unwrap().clone();
            for item in &monkey.3 {
                let item = inspect_item(item.clone(), &monkey.0, &lcm).unwrap();
                if item % monkey.1 == 0 {
                    monkeys.get_mut(monkey.2.0).unwrap().3.push(item);
                } else {
                    monkeys.get_mut(monkey.2.1).unwrap().3.push(item);
                }
            }
            // remove all items and update the number of inspected
            let monkey = monkeys.get_mut(i).unwrap();
            monkey.4 += monkey.3.len() as u64;
            monkey.3 = Vec::new();
        }
    }
    monkeys.sort_by_key(|m| m.4);
    monkeys.reverse();
    let monkey_business = monkeys.get(0).unwrap().4 * monkeys.get(1).unwrap().4;
    println!("{}", monkey_business);
}

fn inspect_item(item: u64, operation: &String, lcm: &u64) -> Option<u64> {
    if let Number(value) = Expr::new(operation).value("old", item).exec().unwrap() {
        if IS_PART_2 {
            return Some(value.as_u64().unwrap() % lcm);
        }
        return Some(value.as_u64().unwrap() / 3);
    }
    None
}

#[derive(Clone)]
// (operation, divider, (monkey if true, monkey if false), list of items, number of inspected items)
struct Monkey(String, u64, (usize, usize), Vec<u64>, u64);

impl Monkey {
    fn parse(chunk: &[&str]) -> Monkey {
        let starting_items = chunk[1].split(':').collect::<Vec<&str>>()[1].split(',').collect::<Vec<&str>>().iter().map(|s| s.trim().parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let operation = chunk[2].split("= ").collect::<Vec<&str>>()[1].to_string();
        let divider = chunk[3].split("by ").collect::<Vec<&str>>()[1].parse::<u64>().unwrap();
        let if_true = chunk[4].split("monkey ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
        let if_false = chunk[5].split("monkey ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
        Monkey(operation, divider, (if_true, if_false), starting_items, 0)
    }
}