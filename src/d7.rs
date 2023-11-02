//! A solution to day 7 year 2021.
//! https://adventofcode.com/2021/day/7

use crate::answer::Answer;

type Parsed = Vec<i64>;

pub fn parse(input: String) -> Parsed {
    let mut positions: Vec<i64> = input
        .split(',')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    positions.sort();

    positions
}

pub fn part1(positions: Parsed) -> impl Answer {
    let median = positions.get(positions.len() / 2).unwrap();
    let fuel: i64 = positions.iter().map(|pos| (pos - median).abs()).sum();

    fuel
}

pub fn part2(positions: Parsed) -> impl Answer {
    let mean: i64 = positions.iter().sum::<i64>() / (positions.len() as i64);

    let fuel: i64 = positions
        .iter()
        .map(|pos| {
            let d = (pos - mean).abs();
            (d.pow(2) + d) / 2
        })
        .sum();

    fuel
}
