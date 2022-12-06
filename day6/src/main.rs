///////////////////////////  AOC 2022 - Day 6 - Rust ///////////////////////////

use std::{include_str, collections::HashSet};

////////////////////////////////////////////////////////////////////////////////

fn find_sequence (signal: &Vec<char>, of_size: usize) -> () {

    let mut buffer: HashSet<char> = HashSet::with_capacity(of_size);

    for index in 0..signal.len() {

        if let Some(x) = signal.get(index..index + of_size) {
            for char in x { buffer.insert(*char); }
            if buffer.len() == of_size {
                println!("Found sequence in array at position: {}", index + of_size);
                break
            }
        }

        buffer.clear()
    }
}

fn main () -> () {
    let signal: Vec<char> = include_str!("data/data.txt").chars().collect();
    find_sequence(&signal, 4);
    find_sequence(&signal, 14);
}

////////////////////////////////////////////////////////////////////////////////
