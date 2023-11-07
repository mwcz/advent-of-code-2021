//! A solution to day 8 year 2021.
//! https://adventofcode.com/2021/day/8

use std::collections::HashMap;

type Model = String;
type Answer = usize;

pub fn parse(input: String) -> Model {
    input
}

pub fn part1(input: Model) -> Answer {
    let digits: usize = input
        .lines()
        .map(|line| {
            line.split('|')
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|digit| digit.to_string())
                .filter(|digit| [2, 3, 4, 7].contains(&digit.len()))
                .collect::<Vec<_>>()
                .len()
        })
        .sum();

    digits
}

pub fn part2(input: Model) -> String {
    let alpha_to_num = HashMap::from([
        ('a', 0),
        ('b', 1),
        ('c', 2),
        ('d', 3),
        ('e', 4),
        ('f', 5),
        ('g', 6),
    ]);

    // For each number 0-9, 7 bits that identify which wires must be turned on to light up that
    // number.
    let wires = [
        0b1110111, // "0"
        0b0010010, // "1"
        0b1011101, // "2"
        0b1011011, // "3"
        0b0111010, // "4"
        0b1101011, // "5"
        0b1101111, // "6"
        0b1010010, // "7"
        0b1111111, // "8"
        0b1111011, // "9"
    ];

    // The wire count needed to light up each number.
    let num_to_wire_count = [
        6, // "0"
        2, // "1"
        5, // "2"
        5, // "3"
        4, // "4"
        5, // "5"
        6, // "6"
        3, // "7"
        7, // "8"
        6, // "9"
    ];

    // acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf

    let lines = input.lines();

    // possibility space for numbers
    let mut num_space: [Vec<u8>; 10] = [
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    let mut wire_space = [-1; 8];

    let test_input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    let input: Vec<Vec<&str>> = test_input
        .split('|')
        .map(|l| l.trim().split_whitespace().collect::<Vec<&str>>())
        .collect();

    dbg!(&input);

    let with_uniq_lens: Vec<&str> = input[0]
        .iter()
        .filter(|digit| [2, 3, 4].contains(&digit.len()))
        .cloned()
        .collect();

    for seq in &with_uniq_lens {
        let len = seq.len();
        let (num, _) = num_to_wire_count
            .iter()
            .enumerate()
            .find(|(_i, &c)| c == len)
            .unwrap();
        let nspace = num_space[num].clone();
        dbg!(&nspace);
        // seq.chars().for_each(|c| alpha_to_num.get(c).unwrap());
    }

    dbg!(&with_uniq_lens);

    "incomplete".to_string()
}
