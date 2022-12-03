///////////////////////////  AOC 2022 - Day 3 - Rust ///////////////////////////

use std::{include_str, collections::HashSet};

////////////////////////////////////////////////////////////////////////////////

fn get_priority (letter: &char) -> u32 {

    if letter.is_uppercase() {
        *letter as u32 - 38
    }
    else {
        *letter as u32 - 96
    }
}

fn main() {

    let mut part_1_duplicates: Vec<char> = Vec::new();
    let mut part_2_duplicates: Vec<char> = Vec::new();

    let lines: Vec<&str> = include_str!("data/data.txt").split("\n").collect();

    for line_group in lines.chunks(3) {

        let mut group_sets: Vec<HashSet<char>> = Vec::new();

        for line in line_group {

            let halves = line.split_at(line.len() / 2);

            let half_1: HashSet<char> = HashSet::from_iter(halves.0.chars());
            let half_2: HashSet<char> = HashSet::from_iter(halves.1.chars());

            part_1_duplicates.extend(half_1.intersection(&half_2));
            group_sets.push(HashSet::from_iter(line.chars()));

        }

        // Gross, but couldn't get chaining to work quickly
        let x: HashSet<char> = group_sets[0].intersection(&group_sets[1]).copied().collect();
        part_2_duplicates.extend(x.intersection(&group_sets[2]));

    }

    println!("Part 1 score: {}", part_1_duplicates.iter().map(|x| get_priority(x)).sum::<u32>());
    println!("Part 2 score: {}", part_2_duplicates.iter().map(|x| get_priority(x)).sum::<u32>());

}

////////////////////////////////////////////////////////////////////////////////
