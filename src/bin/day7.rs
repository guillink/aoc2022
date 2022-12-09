use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("inputs/day7.txt").expect("should read input");
    let mut cur_dir = Vec::new();
    let mut files = Vec::new();
    for line in input.lines() {
        if line.starts_with("$ cd") {
            match &line[5..] {
                ".." => {
                    cur_dir.pop();
                }
                "/" => {
                    cur_dir.clear();
                }
                dir => {
                    cur_dir.push(dir);
                }
            };
        } else if line.starts_with("$ ls") {
            ()
        } else {
            let parts = line.split_once(' ').expect("should split line");
            if parts.0 != "dir" {
                files.push(File {
                    path: cur_dir.clone(),
                    // name: parts.1.to_owned(),
                    size: parts.0.parse::<u32>().expect("should parse file size"),
                })
            }
        }
    }

    let mut dir_sizes = HashMap::new();
    for file in files {
        dir_sizes
            .entry("".to_string())
            .and_modify(|s| *s += file.size)
            .or_insert(file.size);
        let mut cur_path = Vec::new();
        for dir in file.path {
            cur_path.push(dir);
            dir_sizes
                .entry(cur_path.join("/"))
                .and_modify(|s| *s += file.size)
                .or_insert(file.size);
        }
    }

    let part1 = dir_sizes
        .iter()
        .filter_map(|(_, size)| if size <= &100000 { Some(size) } else { None })
        .sum::<u32>();
    println!("Part 1: {part1}");

    const TOTAL_SIZE: u32 = 70_000_000;
    const NEEDED_SIZE: u32 = 30_000_000;
    let size_to_free =
        NEEDED_SIZE - (TOTAL_SIZE - dir_sizes.get("").expect("should find total used space"));
    let part2 = dir_sizes
        .iter()
        .filter_map(|(_, size)| {
            if size >= &size_to_free {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .expect("should get min size to free");
    println!("Part 2: {part2}");
}

struct File<'a> {
    path: Vec<&'a str>,
    // name: String,
    size: u32,
}
