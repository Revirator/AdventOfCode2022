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
                let (a_11, a_12, a_21, a_22) = (
                    &elf_1[0].parse::<u32>().unwrap(),
                    &elf_1[1].parse::<u32>().unwrap(),
                    &elf_2[0].parse::<u32>().unwrap(),
                    &elf_2[1].parse::<u32>().unwrap()
                );
                if (a_11 >= a_21 && a_12 <= a_22) || (a_21 >= a_11 && a_22 <= a_12) {
                    complete_overlap += 1;
                }
                if a_12 >= a_21 && a_22 >= a_11 {
                    partial_overlap += 1;
                }
            }
        }
        println!("{complete_overlap}\n{partial_overlap}");
    }
}