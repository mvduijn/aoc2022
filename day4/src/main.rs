///////////////////////////  AOC 2022 - Day 4 - Rust ///////////////////////////

use std::include_str;

////////////////////////////////////////////////////////////////////////////////

fn part1 (data: &str) -> usize {

    data
        .split("\n")
        .map(|line| {
            let assignments: Vec<&str> = line.split(",").collect();
            let pairs: Vec<usize> = assignments.iter()
                .flat_map(|assignment| assignment.split("-"))
                .map(|val| val.parse::<usize>().unwrap())
                .collect();
            pairs.chunks(2).map(|x| x.to_vec()).collect::<Vec<Vec<usize>>>()
        })
        .fold(0, |total, x| {
            let matches =  x[0][0] >= x[1][0] && x[0][1] <= x[1][1] ||
                           x[1][0] >= x[0][0] && x[1][1] <= x[0][1];
            total + matches as usize
        })
}

fn part2 (data: &str) -> usize {

    data
        .split("\n")
        .map(|line| {
            let assignments: Vec<&str> = line.split(",").collect();
            let pairs: Vec<usize> = assignments.iter()
                .flat_map(|assignment| assignment.split("-"))
                .map(|val| val.parse::<usize>().unwrap())
                .collect();
            pairs.chunks(2).map(|x| x.to_vec()).collect::<Vec<Vec<usize>>>()
        })
        .fold(0, |total, x| {
            let matches =  x[0][0] <= x[1][0] && x[0][1] >= x[1][0] ||
                           x[1][0] <= x[0][0] && x[1][1] >= x[0][0];
            total + matches as usize
        })
}

fn main () {
    let data = include_str!("data/data.txt");
    println!("Part 1 value: {:?}", part1(data));
    println!("Part 2 value: {:?}", part2(data));
}

////////////////////////////////////////////////////////////////////////////////
