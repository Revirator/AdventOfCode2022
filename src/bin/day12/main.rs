use advent_of_code::reader;

fn main() {
    if let Ok(lines) = reader::read_input_lines("./src/bin/day12/input.txt") {
        let mut start = (0, 0);
        let mut end = (0, 0);
        // (char, visited, distance from start)
        let mut maze: Vec<Vec<(u8, bool, i32)>> = Vec::new();
        for (row, line) in lines.enumerate() {
            if let Ok(ip) = line {
                let mut row_cells: Vec<(u8, bool, i32)> = Vec::new();
                let bytes = ip.into_bytes();
                for column in 0..bytes.len() {
                    let byte = *bytes.get(column).unwrap();
                    if byte == 83 {
                        start = (row, column);
                        row_cells.push(('a' as u8, true, 0));
                        continue;
                    }
                    if byte == 69 {
                        end = (row, column);
                        row_cells.push(('z' as u8, false, 0));
                        continue;
                    }
                    row_cells.push((byte, false, 0));
                }
                maze.push(row_cells);
            }
        }
        let shortest_path_from_start = solve_from_start(start, end, maze.clone());
        let shortest_path_from_end = solve_from_end(end, maze.clone());
        println!("{}\n{}", shortest_path_from_start, shortest_path_from_end);
    }
}

fn solve_from_start(start: (usize, usize), end: (usize, usize), mut maze: Vec<Vec<(u8, bool, i32)>>) -> i32 {
    let mut queue: Vec<(usize, usize)> = Vec::new();
    queue.push(start);
    while !queue.is_empty() {
        let position = queue.remove(0);
        let cell = maze.get(position.0).unwrap().get(position.1).unwrap().clone();
        if position == end {
            return cell.2;
        }
        if position.0 > 0 {
            let cell_above = maze.get_mut(position.0 - 1).unwrap().get_mut(position.1).unwrap();
            if !cell_above.1 && cell_above.0 - 1 <= cell.0 {
                cell_above.1 = true;
                cell_above.2 = cell.2 + 1;
                queue.push((position.0 - 1, position.1));
            }
        }
        if position.0 + 1 < maze.len() {
            let cell_below = maze.get_mut(position.0 + 1).unwrap().get_mut(position.1).unwrap();
            if !cell_below.1 && cell_below.0 - 1 <= cell.0 {
                cell_below.1 = true;
                cell_below.2 = cell.2 + 1;
                queue.push((position.0 + 1, position.1));
            }
        }
        if position.1 > 0 {
            let cell_left = maze.get_mut(position.0).unwrap().get_mut(position.1 - 1).unwrap();
            if !cell_left.1 && cell_left.0 - 1 <= cell.0 {
                cell_left.1 = true;
                cell_left.2 = cell.2 + 1;
                queue.push((position.0, position.1 - 1));
            }
        }
        if position.1 + 1 < maze.get(position.0).unwrap().len() {
            let cell_right = maze.get_mut(position.0).unwrap().get_mut(position.1 + 1).unwrap();
            if !cell_right.1 && cell_right.0 - 1 <= cell.0 {
                cell_right.1 = true;
                cell_right.2 = cell.2 + 1;
                queue.push((position.0, position.1 + 1));
            }
        }
    }
    return i32::MAX;
}

fn solve_from_end(end: (usize, usize), mut maze: Vec<Vec<(u8, bool, i32)>>) -> i32 {
    let mut queue: Vec<(usize, usize)> = Vec::new();
    queue.push(end);
    while !queue.is_empty() {
        let position = queue.remove(0);
        let cell = maze.get(position.0).unwrap().get(position.1).unwrap().clone();
        if cell.0 == 97 {
            return cell.2;
        }
        if position.0 > 0 {
            let cell_above = maze.get_mut(position.0 - 1).unwrap().get_mut(position.1).unwrap();
            if !cell_above.1 && cell_above.0 + 1 >= cell.0 {
                cell_above.1 = true;
                cell_above.2 = cell.2 + 1;
                queue.push((position.0 - 1, position.1));
            }
        }
        if position.0 + 1 < maze.len() {
            let cell_below = maze.get_mut(position.0 + 1).unwrap().get_mut(position.1).unwrap();
            if !cell_below.1 && cell_below.0 + 1 >= cell.0 {
                cell_below.1 = true;
                cell_below.2 = cell.2 + 1;
                queue.push((position.0 + 1, position.1));
            }
        }
        if position.1 > 0 {
            let cell_left = maze.get_mut(position.0).unwrap().get_mut(position.1 - 1).unwrap();
            if !cell_left.1 && cell_left.0 + 1 >= cell.0 {
                cell_left.1 = true;
                cell_left.2 = cell.2 + 1;
                queue.push((position.0, position.1 - 1));
            }
        }
        if position.1 + 1 < maze.get(position.0).unwrap().len() {
            let cell_right = maze.get_mut(position.0).unwrap().get_mut(position.1 + 1).unwrap();
            if !cell_right.1 && cell_right.0 + 1 >= cell.0 {
                cell_right.1 = true;
                cell_right.2 = cell.2 + 1;
                queue.push((position.0, position.1 + 1));
            }
        }
    }
    return i32::MAX;
}