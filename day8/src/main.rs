///////////////////////////  AOC 2022 - Day 5 - Rust ///////////////////////////

use std::cmp::Ordering;

use grid::Grid;

////////////////////////////////////////////////////////////////////////////////

enum Direction {
    Left,
    Right,
    Up,
    Down
}

fn get_directions (grid: &Grid<u32>, row: usize, pos: usize) -> Vec<(Direction, Vec<u32>)> {

    let left = grid[row][..pos].to_vec();
    let right = grid[row][pos + 1..].to_vec();
    let up = grid.iter_col(pos).take(row).cloned().collect();
    let down = grid.iter_col(pos).skip(row + 1).cloned().collect();

    vec![(Direction::Left, left), (Direction::Right, right), (Direction::Up, up), (Direction::Down, down)]
}

fn generate_grid () -> Grid<u32> {

    let mut grid: Grid<u32> = Grid::new(0, 0);

    include_str!("data/data.txt")
        .lines()
        .map(|line| line.chars().map(|digit| digit.to_digit(10).unwrap()).collect())
        .for_each(|row| grid.push_row(row));

    grid
}

fn part1 (grid: &Grid<u32>) -> usize {

    let mut visible_counter: usize = (grid.rows() + grid.cols()) * 2 - 4;

    for row in 1..grid.rows() - 1 {
        for column in 1..grid.cols() - 1 {
            if get_directions(grid, row, column)
                .iter()
                .map(|sequence| sequence.1.iter().any(|&x| x >= grid[row][column]))
                .any(|invisible| invisible == false)
            {
                visible_counter += 1
            }
        }
    }
    visible_counter
}

fn part2 (grid: &Grid<u32>) -> usize {

    let mut products: Vec<usize> = vec!();

    for row in 0..grid.rows() {
        for column in 0..grid.cols() {

            let target = &grid[row][column];

            let test: Vec<usize> = get_directions(grid, row, column)
                .iter()
                .map(|val| {
                    let mut counter = 0;
                    let (direction, sequence) = val;
                    let mut numbers = sequence.clone();

                    match direction {
                        Direction::Left | Direction::Up => { numbers = sequence.iter().rev().copied().collect(); }
                        _ => ()
                    }

                    for nr in &numbers {
                        match nr.cmp(target) {
                            Ordering::Less => { counter += 1 }
                            Ordering::Equal | Ordering::Greater => { counter += 1; break }
                        }
                    }
                    counter
                })
                .collect();

            products.push(test.iter().product());
        }
    }
    *products.iter().max().unwrap()
}

fn main () -> () {
    let grid = generate_grid();
    println!("Part 1 value: {}", part1(&grid));
    println!("Part 2 value: {}", part2(&grid));
}

////////////////////////////////////////////////////////////////////////////////
