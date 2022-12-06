use advent_of_code::reader;

fn main() {
    if let Ok(lines) = reader::read_input_lines("./src/bin/day1/input.txt") {
        let mut max_calories = [-1, -1, -1];
        let mut curr_calories = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    update_calories(&mut max_calories, curr_calories);
                    curr_calories = 0;
                    continue;
                }
                curr_calories += ip.parse::<i32>().unwrap();
            }
        }
        update_calories(&mut max_calories, curr_calories);
        println!("{}", max_calories[2]);
        println!("{}", max_calories[0] + max_calories[1] + max_calories[2])
    }
}

fn update_calories(arr: &mut [i32; 3], val: i32) {
    if val > arr[0] {
        if val > arr[1] {
            if val > arr[2] {
                arr[0] = arr[1];
                arr[1] = arr[2];
                arr[2] = val;
            } else {
                arr[0] = arr[1];
                arr[1] = val;
            }
        } else {
            arr[0] = val;
        }
    }
}