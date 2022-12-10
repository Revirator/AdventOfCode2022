use advent_of_code::reader;

fn main() {
    if let Ok(lines) = reader::read_input_lines("./src/bin/day10/input.txt") {
        let (mut register, mut sum, mut cycle) = (1, 0, 0);
        let mut pixels = String::from("");
        for line in lines {
            if let Ok(ip) = line {
                if ip == "noop" {
                    process_cycle(&mut sum, &mut pixels, &mut cycle, register);
                } else {
                    let n = ip.split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
                    process_cycle(&mut sum, &mut pixels, &mut cycle, register);
                    process_cycle(&mut sum, &mut pixels, &mut cycle, register);
                    register += n;
                }
            }
        }
        println!("{}\n{}", sum, pixels);
    }
}

fn process_cycle(sum: &mut i32, pixels: &mut String, cycle: &mut i32, register: i32) {
    update_pixels(pixels, *cycle, register);
    *cycle += 1;
    if *cycle % 40 == 20 {
        *sum += register * *cycle;
    }
}

fn update_pixels(pixels: &mut String, cycle: i32, register: i32) {
    let cycle = cycle % 40;
    if cycle >= register - 1 && cycle <= register + 1 {
        pixels.push('#');
    } else {
        pixels.push('.');
    }
    if cycle == 39 {
        pixels.push('\n');
    }
}
