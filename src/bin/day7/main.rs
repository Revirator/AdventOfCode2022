use advent_of_code::reader;

fn main() {
    if let Ok(mut lines) = reader::read_input_lines("./src/bin/day7/input.txt") {
        let mut dirs: Vec<(i64, usize)> = Vec::new();
        dirs.push((0, 0));
        // (size, parent)
        let mut curr_dir = dirs.get_mut(0).unwrap();
        let mut curr_dir_index = 0;
        // skip first line
        lines.next();
        let mut curr_line = lines.next().unwrap().unwrap();
        while curr_line != "" {
            if &curr_line[2..4] == "cd" {
                let name = &curr_line[5..];
                if name == ".." {
                    curr_dir_index = curr_dir.1;
                    curr_dir = dirs.get_mut(curr_dir_index).unwrap();
                } else {
                    dirs.push((0, curr_dir_index));
                    curr_dir_index = dirs.len() - 1;
                    curr_dir = dirs.get_mut(curr_dir_index).unwrap();
                }
                curr_line = lines.next().unwrap_or(Ok(String::from(""))).unwrap();
            } else {
                curr_line = lines.next().unwrap_or(Ok(String::from(""))).unwrap();
                while curr_line != "" && !curr_line.starts_with('$') {
                    let temp: Vec<&str> = curr_line.split(" ").collect();
                    let size = temp.get(0).unwrap().clone();
                    if size != "dir" {
                        curr_dir.0 += size.parse::<i64>().unwrap();
                    }
                    curr_line = lines.next().unwrap_or(Ok(String::from(""))).unwrap();
                }
            }
        }
        // (from, to)
        let mut updates: Vec<(usize, usize)> = Vec::new();
        for i in 1..dirs.len() {
            updates.push((i, dirs.get(i).unwrap().1));
        }
        updates.sort_by_key(|t| t.1);
        updates.reverse();
        for (from, to) in updates {
            let size = dirs.get(from).unwrap().0;
            dirs.get_mut(to).unwrap().0 += size;
        }
        let (mut sum_small_dirs, mut dir_size_to_delete) = (0, i64::MAX);
        let size_to_delete = dirs.get(0).unwrap().0 - 40_000_000;
        for dir in dirs {
            if dir.0 <= 100_000 {
                sum_small_dirs += dir.0;
            }
            if dir.0 >= size_to_delete && dir.0 < dir_size_to_delete {
                dir_size_to_delete = dir.0;
            }
        }
        println!("{}\n{}", sum_small_dirs, dir_size_to_delete);
    }
}