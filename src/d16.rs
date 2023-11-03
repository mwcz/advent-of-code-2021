//! A solution to day 16 year 2021.
//! https://adventofcode.com/2021/day/16

use crate::answer::Answer;

type Parsed = Vec<u8>;

pub fn parse(input: String) -> Parsed {
    input
        .trim()
        .chars()
        .map(from_hex)
        .collect::<Vec<&str>>()
        .join("")
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect()
}

pub fn part1(mut msg: Parsed) -> impl Answer {
    // "program counter"
    let mut pc = 0;
    let mut done = false;

    // while !done {
    // read version and type from
    // let version = Vec::from(msg[pc..3];
    match msg[(pc + 3)..(pc + 6)] {
        [1, 0, 0] => println!("its four!"),
        _ => println!("it's not four :("),
    }
    // }

    "incomplete"
}

pub fn part2(input: Parsed) -> impl Answer {
    "incomplete"
}

fn from_hex<'a>(c: char) -> &'a str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("{c} is not a valid hex char"),
    }
}
