#![feature(stdin_forwarders)]
use std::{
    fmt::Display,
    io::{self, BufRead},
};

fn main() {
    let mut lines = io::stdin().lines();

    // parse the numbers on the first line
    let numbers: Vec<u32> = lines
        .next() // get the first line
        .unwrap() // unwrap Option
        .unwrap() // unwrap Result
        .split(",")
        .map(|n| n.parse::<u32>().unwrap()) // parse to u32
        .collect();

    // skip the guaranteed empty line
    lines.next();

    // parse the boards text into a workable data structure
    let boards: Vec<Vec<Vec<u32>>> = lines
        .map(|r| r.unwrap()) // unwrap Result
        .collect::<Vec<String>>() // collect into a Vec of Strings, couldn't figure out how to split a std::io::Lines
        .split(|line| line.len() == 0) // split on empty lines
        .map(|board| {
            board
                .iter()
                .map(|row| {
                    row.trim()
                        .to_string()
                        .split_whitespace()
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    println!("numbers:");
    println!("  {:?}\n", numbers);

    println!("boards ({} found)", boards.len());
    for board in &boards {
        for row in board {
            println!("  {:?}", row);
        }
        println!("");
    }
}
