use std::io;

mod part1;
mod part2;

fn read_stdin_lines() -> Vec<String> {
    io::stdin().lines().collect::<Result<_, _>>().unwrap()
}

fn main() {
    let in_lines = read_stdin_lines();

    println!(
        "Day 3: Rucksack Reorganization\nPart One: {}\nPart Two: {}",
        part1::solve(&in_lines),
        part2::solve(&in_lines)
    );
}
