use advent_of_code::reader;

fn main() {
    if let Ok(lines) = reader::read_input_lines("./src/bin/day8/input.txt") {
        let mut grid: Vec<Vec<u8>> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                grid.push(ip.into_bytes())
            }
        }
        let (height, width) = (grid.len(), grid.get(0).unwrap().len());
        let mut visible_trees = 2 * (height + width) - 4;
        let mut max_scenic_score = 0;
        for i in 1..height - 1 {
            for j in 1..width - 1 {
                let mut check = true;
                let mut flag = 0;
                let curr = grid.get(i).unwrap().get(j).unwrap().clone();
                // check if visible from the left
                let row = grid.get(i).unwrap();
                let mut trees_left = 0;
                for m in (0..j).rev() {
                    if row.get(m).unwrap() < &curr {
                        flag += 1;
                        trees_left += 1;
                    }
                    if row.get(m).unwrap() >= &curr {
                        trees_left += 1;
                        break;
                    }
                }
                if flag == j && check {
                    visible_trees += 1;
                    check = false;
                }
                flag = 0;
                // check if visible from the right
                let mut trees_right = 0;
                for m in j + 1..width {
                    if row.get(m).unwrap() < &curr {
                        flag += 1;
                        trees_right += 1;
                    }
                    if row.get(m).unwrap() >= &curr {
                        trees_right += 1;
                        break;
                    }
                }
                if flag == width - j - 1 && check {
                    visible_trees += 1;
                    check = false;
                }
                flag = 0;
                // check if visible from the top
                let mut trees_top = 0;
                for m in (0..i).rev() {
                    if grid.get(m).unwrap().get(j).unwrap() < &curr {
                        flag += 1;
                        trees_top += 1;
                    }
                    if grid.get(m).unwrap().get(j).unwrap() >= &curr {
                        trees_top += 1;
                        break;
                    }
                }
                if flag == i && check {
                    visible_trees += 1;
                    check = false;
                }
                flag = 0;
                // check if visible from the bottom
                let mut trees_bottom = 0;
                for m in i + 1..height {
                    if grid.get(m).unwrap().get(j).unwrap() < &curr {
                        flag += 1;
                        trees_bottom += 1;
                    }
                    if grid.get(m).unwrap().get(j).unwrap() >= &curr {
                        trees_bottom += 1;
                        break;
                    }
                }
                if flag == height - i - 1 && check {
                    visible_trees += 1;
                }
                let scenic_score = trees_left * trees_right * trees_top * trees_bottom;
                if scenic_score > max_scenic_score {
                    max_scenic_score = scenic_score;
                }
            }
        }
        println!("{}\n{}", visible_trees, max_scenic_score);
    }
}