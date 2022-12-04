#[path = "../reader.rs"]
mod reader;

fn main() {
    if let Ok(lines) = reader::read_lines("./src/bin/day2/input.txt") {
        let (mut score_1, mut score_2) = (0, 0);
        for line in lines {
            if let Ok(ip) = line {
                let chars: Vec<char> = ip.chars().collect();
                let elf = chars[0] as i32;
                let player = chars[2] as i32;
                score_1 += player - 87 + ((player - elf + 2) % 3) * 3;
                score_2 += (player - 88 + elf - 65 + 2) % 3 + 1 + (player - 88) * 3;
            }
        }
        println!("{}\n{}", score_1, score_2);
    }
}