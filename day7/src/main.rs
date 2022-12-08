///////////////////////////  AOC 2022 - Day 7 - Rust ///////////////////////////

use itertools::Itertools;
use std::{collections::HashMap};

////////////////////////////////////////////////////////////////////////////////

struct File {
    name: String,
    size: u32,
}

struct Directory {
    name: String,
    files: HashMap<String, File>,
    directories: HashMap<String, Directory>,
}

impl Directory {

    fn get_files_size (&self) -> u32 {
        self.files.values().map(|file| file.size).sum()
    }

    fn get_total_size (&self) -> u32 {

        let mut size = self.get_files_size();

        for directory in get_all_directories_in(self) {
            size += directory.get_files_size()
        }

        size
    }

}

struct FileSystem {
    directories: HashMap<String, Directory>,
}

enum TerminalLine {
    List,
    ChangeDirectory(String),
    File(File),
    Directory(Directory),
}

fn get_current_directory <'a> (file_system: &'a mut FileSystem, pwd: &Vec<String>) -> &'a mut Directory {

    let mut curdir = file_system.directories.get_mut(pwd.first().unwrap()).unwrap();

    for directory in pwd.iter().skip(1) {
        curdir = curdir.directories.get_mut(directory).unwrap()
    }

    curdir
}

fn get_all_directories_in (directory: &Directory) -> Vec<&Directory> {

    let mut directories: Vec<&Directory> = vec!();

    for directory in directory.directories.values() {
        directories.push(directory);
        directories.append(&mut get_all_directories_in(directory))
    }

    directories
}

fn parse_terminal_line (line: &str) -> TerminalLine {

    match &line[0..4] {
        "$ cd" => TerminalLine::ChangeDirectory(line[5..].to_string()),
        "$ ls" => TerminalLine::List,
        "dir " => TerminalLine::Directory(Directory{ name: line[4..].to_string(), files: HashMap::new(), directories: HashMap::new()}),
        _ => {
                let (size, name) = line.split(" ").tuples().next().unwrap();
                let file = File { size: size.parse().unwrap(), name: name.to_string() };
                TerminalLine::File(file)
        }
    }
}

fn generate_filesystem () -> FileSystem {

    let lines: Vec<&str> = include_str!("data/data.txt").lines().collect();
    let mut file_system: FileSystem = FileSystem { directories: HashMap::new() };

    let mut pwd: Vec<String> = vec!(String::from("/"));

    let root: Directory = Directory{ name: String::from("/"), files: HashMap::new(), directories: HashMap::new()};
    file_system.directories.insert(root.name.to_string(), root);

    for line in lines.iter().skip(2) {

        match parse_terminal_line(line) {
            TerminalLine::List => {},
            TerminalLine::ChangeDirectory(name) => {
                if name == ".." {
                    pwd = pwd[0..pwd.len() - 1].to_vec()
                } else {
                    pwd.push(name)
                }
            }
            TerminalLine::File(file) => {
                get_current_directory(&mut file_system, &pwd).files.insert(file.name.to_string(), file);
            }
            TerminalLine::Directory(directory) => {
                get_current_directory(&mut file_system, &pwd).directories.insert(directory.name.to_string(), directory);
            }
        }
    }
    file_system
}

fn part1 (file_system: &FileSystem) -> u32 {

    let mut all_directories = vec!(&file_system.directories["/"]);
    all_directories.append(&mut get_all_directories_in(&all_directories[0]));

    all_directories
        .iter()
        .map(|directory| directory.get_total_size())
        .filter(|size| *size <= 100000 as u32)
        .sum()

}

fn part2 (file_system: &FileSystem) -> u32 {

    let mut all_directories = vec!(&file_system.directories["/"]);
    all_directories.append(&mut get_all_directories_in(&all_directories[0]));

    let unused_space = 70000000 - &file_system.directories["/"].get_total_size();

    all_directories.iter()
        .map(|directory| directory.get_total_size())
        .filter(|size| *size + unused_space >= 30000000)
        .min()
        .unwrap()

}

fn main () -> () {
    let file_system: FileSystem = generate_filesystem();
    println!("The answer to Part 1 is {}", part1(&file_system));
    println!("The answer to Part 2 is {}", part2(&file_system));
}

////////////////////////////////////////////////////////////////////////////////
