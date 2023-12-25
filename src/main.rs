use std::io;

fn main() {
    println!("Input the grid in on 4 lines");
    let grid = io::stdin()
        .lines()
        .take(4)
        .map(|l| l.expect("Not enough lines").chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let found_vec = sanamahti::solve(grid);
    println!("Found the following words");
    println!("{}", found_vec.join("\n"));
}
