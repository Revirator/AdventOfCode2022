mod one;
mod two;

fn main() {
    let solution_1 = one::solve().unwrap();
    let solution_2 = two::solve().unwrap();
    println!("{solution_1}\n{solution_2}")
}