use std::{fs::read_to_string, str::FromStr};

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(format!("unrecognized input {s}")),
        }
    }
}

impl Shape {
    fn from_outcome(wanted_outcome: &Outcome, other: &Shape) -> Self {
        match (wanted_outcome, other) {
            (Outcome::Win, Shape::Rock) => Shape::Paper,
            (Outcome::Win, Shape::Paper) => Shape::Scissors,
            (Outcome::Win, Shape::Scissors) => Shape::Rock,
            (Outcome::Draw, Shape::Rock) => Shape::Rock,
            (Outcome::Draw, Shape::Paper) => Shape::Paper,
            (Outcome::Draw, Shape::Scissors) => Shape::Scissors,
            (Outcome::Lose, Shape::Rock) => Shape::Scissors,
            (Outcome::Lose, Shape::Paper) => Shape::Rock,
            (Outcome::Lose, Shape::Scissors) => Shape::Paper,
        }
    }

    fn versus(&self, other: &Shape) -> Outcome {
        match (self, other) {
            (Shape::Rock, Shape::Rock) => Outcome::Draw,
            (Shape::Rock, Shape::Paper) => Outcome::Lose,
            (Shape::Rock, Shape::Scissors) => Outcome::Win,
            (Shape::Paper, Shape::Rock) => Outcome::Win,
            (Shape::Paper, Shape::Paper) => Outcome::Draw,
            (Shape::Paper, Shape::Scissors) => Outcome::Lose,
            (Shape::Scissors, Shape::Rock) => Outcome::Lose,
            (Shape::Scissors, Shape::Paper) => Outcome::Win,
            (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        }
    }

    fn score(&self) -> u16 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(format!("unrecognized input {s}")),
        }
    }
}

impl Outcome {
    fn score(&self) -> u16 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

fn main() {
    let content = read_to_string("inputs/day2.txt").expect("should read input");
    let guide: Vec<(Shape, Shape)> = content
        .split('\n')
        .map(|line| {
            let (opponent, me) = line.split_once(' ').expect("should split line");
            (
                opponent
                    .parse::<Shape>()
                    .expect("should parse opponent shape"),
                me.parse::<Shape>().expect("should parse my shape"),
            )
        })
        .collect();

    let part1: u16 = guide
        .iter()
        .map(|(opponent, me)| me.versus(opponent).score() + me.score())
        .sum();
    println!("Part1: {part1}");

    let guide = content.split('\n').map(|line| {
        let (opponent, outcome) = line.split_once(' ').expect("should split line");
        (
            opponent
                .parse::<Shape>()
                .expect("should parse opponent shape"),
            outcome
                .parse::<Outcome>()
                .expect("should parse desired outcome"),
        )
    });
    let part2: u16 = guide
        .map(|(opponent, outcome)| {
            outcome.score() + Shape::from_outcome(&outcome, &opponent).score()
        })
        .sum();
    println!("Part2: {part2}");
}
