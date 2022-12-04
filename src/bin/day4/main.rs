#[path = "../reader.rs"]
mod reader;

fn main() {
    if let Ok(lines) = reader::read_lines("./src/bin/day4/input.txt") {
        let (mut complete_overlap, mut partial_overlap) = (0, 0);
        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<&str> = ip.split(',').collect();
                let elf_1: Vec<&str> = split[0].split('-').collect();
                let elf_2: Vec<&str> = split[1].split('-').collect();
                let (s_11, s_12, s_21, s_22) = (
                    &elf_1[0].parse::<u32>().unwrap(),
                    &elf_1[1].parse::<u32>().unwrap(),
                    &elf_2[0].parse::<u32>().unwrap(),
                    &elf_2[1].parse::<u32>().unwrap()
                );
                if (s_11 >= s_21 && s_12 <= s_22) || (s_21 >= s_11 && s_22 <= s_12) {
                    complete_overlap += 1;
                }
                if (s_11 >= s_21 && s_11 <= s_22) || (s_12 >= s_21 && s_12 <= s_22)
                    || (s_21 >= s_11 && s_21 <= s_12) || (s_22 >= s_11 && s_22 <= s_12) {
                    partial_overlap += 1;
                }
            }
        }
        println!("{complete_overlap}\n{partial_overlap}");
    }
}