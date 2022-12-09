use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let part1 = read_to_string("inputs/day3.txt")
        .expect("should read input")
        .lines()
        .map(|line| {
            let (fst, snd) = line.split_at(line.len() / 2);
            HashSet::intersection(
                &fst.chars().collect::<HashSet<_>>(),
                &snd.chars().collect::<HashSet<_>>(),
            )
            .map(priority)
            .sum::<u32>()
        })
        .sum::<u32>();
    println!("Part1: {part1}");

    let part2 = read_to_string("inputs/day3.txt")
        .expect("should read input")
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|lines| {
            lines
                .into_iter()
                .map(|l| l.chars().collect::<HashSet<_>>())
                .reduce(|acc, cur| HashSet::intersection(&acc, &cur).map(|&c| c).collect())
                .expect("should get group badge")
                .iter()
                .map(priority)
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("Part2: {part2}")
}

fn priority(c: &char) -> u32 {
    if c.is_uppercase() {
        (*c as u32) - 38
    } else {
        (*c as u32) - 96
    }
}
