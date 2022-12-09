use std::fs::read_to_string;

fn main() {
    let signal = read_to_string("inputs/day6.txt").expect("should read input");
    println!("Part1: {:?}", find_marker(&signal, 4));
    println!("Part2: {:?}", find_marker(&signal, 14));
}

fn find_marker(signal: &str, len: usize) -> usize {
    signal
        .as_bytes()
        .windows(len)
        .position(|buf| !buf.iter().enumerate().any(|(i, c)| buf[..i].contains(c)))
        .expect("should find marker")
        + len
}
