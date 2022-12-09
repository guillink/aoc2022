use std::{collections::VecDeque, convert::identity, fs::read_to_string, iter::repeat};

fn main() {
    let content = read_to_string("inputs/day5.txt").expect("should read input");
    let (mut crates, instructions) = parse(&content);

    for (qty, from, to) in instructions {
        for _ in 0..qty {
            let crate_from = crates
                .get_mut(from as usize - 1)
                .expect("should get stack to move from")
                .pop_front()
                .expect("should pop crate");
            crates
                .get_mut(to as usize - 1)
                .expect("should get stack to move to")
                .push_front(crate_from);
        }
    }
    let part1: String = crates
        .iter()
        .map(|stack| stack.front())
        .filter_map(identity)
        .collect();
    println!("Part1: {part1}");

    let (mut crates, instructions) = parse(&content);
    for (qty, from, to) in instructions {
        for i in 1..=qty {
            let crate_from = crates
                .get_mut(from as usize - 1)
                .expect("should get stack to move from")
                .remove((qty - i) as usize)
                .expect("should pop crate");
            crates
                .get_mut(to as usize - 1)
                .expect("should get stack to move to")
                .push_front(crate_from);
        }
    }
    let part2: String = crates
        .iter()
        .map(|stack| stack.front())
        .filter_map(identity)
        .collect();
    println!("Part2: {part2}");
}

fn parse(content: &str) -> (Vec<VecDeque<char>>, Vec<(u16, u16, u16)>) {
    let (crates, instructions) = content.split_once("\n\n").expect("should split content");
    (parse_crates(crates), parse_instructions(instructions))
}

fn parse_crates(crates: &str) -> Vec<VecDeque<char>> {
    let mut lines = crates.lines();
    let stacks_count = lines
        .next_back()
        .expect("should read last crates line")
        .chars()
        .max()
        .expect("should read stacks count")
        .to_digit(10)
        .expect("should parse stacks count");
    lines.fold(
        repeat(VecDeque::new())
            .take(stacks_count as usize)
            .collect(),
        |mut stacks, l| {
            for (i, c) in l.char_indices().filter(|(_, c)| c.is_alphabetic()) {
                stacks
                    .get_mut(i / 4)
                    .expect("should get stack")
                    .push_back(c)
            }
            stacks
        },
    )
}

fn parse_instructions(instructions: &str) -> Vec<(u16, u16, u16)> {
    instructions
        .lines()
        .map(|l| {
            let mut fragments = l.split(' ');
            (
                fragments
                    .nth(1)
                    .expect("should get quantity")
                    .parse::<u16>()
                    .expect("should parse quantity"),
                fragments
                    .nth(1)
                    .expect("should get from")
                    .parse::<u16>()
                    .expect("should parse from"),
                fragments
                    .nth(1)
                    .expect("should get to")
                    .parse::<u16>()
                    .expect("should parse to"),
            )
        })
        .collect()
}
