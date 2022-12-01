///////////////////////////  AOC 2022 - Day 1 - Rust ///////////////////////////

use std::error::Error;
use std::io::{self, BufRead};
use std::fs::File;

////////////////////////////////////////////////////////////////////////////////

const FILE_PATH: &str = "src/data/calories.txt";

////////////////////////////////////////////////////////////////////////////////

fn get_calories_from_file (file: File) -> Result<Vec<u32>, Box<dyn Error>> {

    let lines = io::BufReader::new(file).lines();

    let mut calories: Vec<u32> = Vec::new();
    let mut current_count = 0;

    for line in lines {

        let cur_line = line?;

        if cur_line.is_empty() {
            calories.push(current_count);
            current_count = 0;
        }

        else {
            current_count += cur_line.parse::<u32>()?;
        }
    }

    if current_count != 0 {
        calories.push(current_count)
    }

    Ok(calories)

}
fn main() -> Result<(), Box<dyn Error>> {

    let mut calories = get_calories_from_file(File::open(FILE_PATH)?)?;

    calories.sort();

    let top_three: Vec<u32> = calories
        .into_iter()
        .rev()
        .take(3)
        .collect();

    println!("The highest calories an elf was carrying for this input was {}", top_three[0]);
    println!("The top three elves were carrying a total of {} calories.", top_three.iter().sum::<u32>());

    Ok(())

}

////////////////////////////////////////////////////////////////////////////////
