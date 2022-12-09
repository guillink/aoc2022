use std::fs::read_to_string;

fn main() {
    let content = read_to_string("inputs/day1.txt").expect("should read input");
    let mut elves: Vec<u32> = content
        .split("\n\n")
        .map(|foods| {
            foods
                .split('\n')
                .map(|cals| cals.parse::<u32>().expect("should be int"))
                .sum()
        })
        .collect();

    let max: &u32 = elves.iter().max().expect("should not be empty");
    println!("Part1: {max}");

    elves.sort();
    let top3: u32 = elves.iter().rev().take(3).sum();
    println!("Part2: {top3}")
}
