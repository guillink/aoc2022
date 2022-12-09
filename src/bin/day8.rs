use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const TEST_DATA: &str = r#"30373
25512
65332
33549
35390"#;

fn main() {
    const RADIX: u32 = 10;
    let input = read_to_string("inputs/day8.txt").expect("should read input");
    // let input = TEST_DATA;
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let trees = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(RADIX).expect("should parse tree height") as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut seen_trees = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            let current_tree = trees[y][x];
            let rays = [
                trees[0..y].iter().map(|row| &row[x]).collect::<Vec<_>>(),
                trees[((y + 1)..height)]
                    .iter()
                    .map(|row| &row[x])
                    .collect::<Vec<_>>(),
                trees[y][0..x].iter().collect::<Vec<_>>(),
                trees[y][(x + 1)..width].iter().collect::<Vec<_>>(),
            ];
            if rays
                .into_iter()
                .any(|ray| &current_tree > ray.into_iter().max().unwrap_or(&-1))
            {
                seen_trees.insert((x, y));
            }
        }
    }
    println!("Part 1: {}", seen_trees.len());

    let mut highest_scenic_score = 0;
    for y in 0..height {
        for x in 0..width {
            let current_tree = trees[y][x];
            let rays: [Vec<&i32>; 4] = [
                trees[0..y].iter().rev().map(|row| &row[x]).collect(),
                trees[y][0..x].iter().rev().collect(),
                trees[y][(x + 1)..width].iter().collect(),
                trees[(y + 1)..height].iter().map(|row| &row[x]).collect(),
            ];
            let scenic_score = rays.into_iter().fold(1, |acc, ray| {
                let mut visible_trees = 0;
                for tree in ray {
                    visible_trees += 1;
                    if tree >= &current_tree {
                        break;
                    }
                }
                acc * visible_trees
            });
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }
    println!("Part 2: {}", highest_scenic_score)
}
