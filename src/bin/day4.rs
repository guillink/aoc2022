use std::fs::read_to_string;

fn main() {
    let assignments: Vec<((u16, u16), (u16, u16))> = read_to_string("inputs/day4.txt")
        .expect("should read input")
        .lines()
        .map(to_assignments)
        .collect();

    let part1 = assignments
        .iter()
        .filter(|(fst, snd)| {
            (fst.0 <= snd.0 && fst.1 >= snd.1) || (snd.0 <= fst.0 && snd.1 >= fst.1)
        })
        .count();
    println!("Part1: {part1}");

    let part2 = assignments
        .iter()
        .filter(|(fst, snd)| {
            fst.0 >= snd.0 && fst.0 <= snd.1
                || fst.1 >= snd.0 && fst.1 <= snd.1
                || snd.0 >= fst.0 && snd.0 <= fst.1
                || snd.1 >= fst.0 && snd.1 <= fst.1
        })
        .count();
    println!("Part2: {part2}")
}

fn to_assignments(line: &str) -> ((u16, u16), (u16, u16)) {
    let pair = line.split_once(',').expect("should split pair");
    (to_assignment(pair.0), to_assignment(pair.1))
}

fn to_assignment(assignment: &str) -> (u16, u16) {
    let (from, to) = assignment.split_once('-').expect("should split assignment");
    (
        from.parse::<u16>().expect("should parse from"),
        to.parse::<u16>().expect("should parse to"),
    )
}
