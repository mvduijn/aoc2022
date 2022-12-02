///////////////////////////  AOC 2022 - Day 2 - Rust ///////////////////////////

use std::include_str;

////////////////////////////////////////////////////////////////////////////////

pub enum Interpretation {
    Part1,
    Part2,
}

////////////////////////////////////////////////////////////////////////////////

fn get_pick_score (pick: &str) -> u8 {
    match pick {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("Invalid input!")
    }
}

fn get_win_val (pick: &str) -> &str {
    match pick {
        "A" => "B",
        "B" => "C",
        "C" => "A",
        _ => panic!("Invalid input!")
    }
}

fn get_lose_val (pick: &str) -> &str {
    match pick {
        "A" => "C",
        "B" => "A",
        "C" => "B",
        _ => panic!("Invalid input!")
    }
}

fn replace_alias <'a> (interpretation: Interpretation, first: &'a str, alias: &'a str) -> &'a str {

    match interpretation {
        Interpretation::Part1 => {
            match alias {
                "X" => "A",
                "Y" => "B",
                "Z" => "C",
                _ => panic!("Invalid input!")
            }
        },
        Interpretation::Part2 => {
            match alias {
                "X" => get_lose_val(first),
                "Y" => first,
                "Z" => get_win_val(first),
                _ => panic!("Invalid input!")
            }
        }
    }
}

fn get_score (interpretation: Interpretation, battle: &Vec<&str>) -> usize {

    let first = battle.first().unwrap();
    let second = replace_alias(interpretation, first, battle.last().unwrap());

    let win_val = get_win_val(first);
    let pick_score: usize = get_pick_score(second).into();

    match second {
        y if y == win_val => 6 + pick_score,
        y if y == *first => 3 + pick_score,
        _ => 0 + pick_score,
    }

}
////////////////////////////////////////////////////////////////////////////////

fn main() {

    let data: Vec<Vec<&str>> = include_str!("data/battles.txt")
        .split("\n")
        .map(|x| x.split(" ").collect())
        .collect();

    let score_1: usize = data
        .iter()
        .map(|x| get_score(Interpretation::Part1, x))
        .sum();

    let score_2: usize = data
        .iter()
        .map(|x| get_score(Interpretation::Part2, x))
        .sum();

    println!("Part 1: Total score was: {score_1}");
    println!("Part 2: Total score was: {score_2}");
}

////////////////////////////////////////////////////////////////////////////////
