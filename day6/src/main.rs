///////////////////////////  AOC 2022 - Day 6 - Rust ///////////////////////////

use std::{include_str, collections::HashSet};

////////////////////////////////////////////////////////////////////////////////

fn find_sequence (signal: &Vec<char>, of_size: usize) -> usize {

    let mut buffer: HashSet<char> = HashSet::with_capacity(of_size);
    let mut return_val: usize = 0;

    for (index, _) in signal.iter().enumerate() {

        let items = signal.get(index..index + of_size);

        if let Some(x) = items {

            let existed: Vec<bool> = x.iter().map(|char| buffer.insert(*char)).collect();

            if !existed.contains(&false) {
                return_val = index + of_size;
                break
            }
        }

        buffer.clear()

    }

    return_val

}

fn main () -> () {
    let signal: Vec<char> = include_str!("data/data.txt").chars().collect();
    println!("Part 1 value: {}", find_sequence(&signal, 4));
    println!("Part 2 value: {}", find_sequence(&signal, 14));
}

////////////////////////////////////////////////////////////////////////////////
