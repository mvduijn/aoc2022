///////////////////////////  AOC 2022 - Day 5 - Rust ///////////////////////////

#[macro_use] extern crate scan_fmt;
use std::{include_str, collections::VecDeque};

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct Input {
    stacks: Vec<VecDeque<char>>,
    instructions: Vec<(usize, usize, usize)>,
}

////////////////////////////////////////////////////////////////////////////////

fn parse_input () -> Input {

    let lines: Vec<&str> = include_str!("data/data.txt").lines().collect();

    let empty_loc: usize = lines
        .iter()
        .position(|line| line.is_empty())
        .ok_or("Invalid input - there is no blank line.").unwrap();

    let stack_amount: usize = lines[empty_loc - 1].split("   ").count();

    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    for _ in 0..stack_amount { // TODO: Find a better way for this
        stacks.push(VecDeque::new());
    }

    let instructions: Vec<(usize, usize, usize)> = lines.iter()
        .skip(empty_loc + 1)
        .filter_map(|line| scan_fmt!(line, "move {d} from {d} to {d}", usize, usize, usize).ok())
        .collect();

    for line in &lines[0..empty_loc - 1] {

        let groups = line.chars().collect::<Vec<char>>();

        for (index, group) in groups.chunks(4).enumerate() {
            if group[1].is_uppercase() {
                stacks[index].push_back(group[1]);
            }
        }
    }

    Input {stacks, instructions}
}

fn part1 (input: Input) -> String {

    let mut input = input;

    for instruction in input.instructions {
        let (amount, from, to) = instruction;
        for _ in 0..amount {
            let val = input.stacks[from - 1].pop_front().unwrap();
            input.stacks[to - 1].push_front(val)
        }
    }

    input.stacks.iter()
        .filter_map(|stack| stack.front())
        .collect()

}

fn part2 (input: Input) -> String {

    let mut input = input;

    for instruction in input.instructions {

        let (amount, from, to) = instruction;
        let mut collection = Vec::new();

        for _ in 0.. amount {
            collection.push(input.stacks[from - 1].pop_front().unwrap());
        }

        collection.reverse();

        for val in &collection {
            input.stacks[to - 1].push_front(*val)
        }
    }

    input.stacks.iter().filter_map(|stack| stack.front()).collect()
}

fn main () -> () {
    let input = parse_input();
    println!("Part 1 value: {}", part1(input.clone()));
    println!("Part 2 value: {}", part2(input.clone()));
}

////////////////////////////////////////////////////////////////////////////////
