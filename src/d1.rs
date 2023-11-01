//! A solution to day 1 year 2021.
//! https://adventofcode.com/2021/day/1

use crate::answer::Answer;

type Parsed = String;

pub fn parse(input: String) -> Parsed {
    input
}

pub fn part1(input: Parsed) -> impl Answer {
    let mut incs = 0;
    let mut last = i64::MAX;

    for line in input.lines() {
        let number = line.trim_end().parse::<i64>().unwrap();

        if number > last {
            incs += 1;
        }

        last = number;
    }

    incs
}

pub fn part2(input: Parsed) -> impl Answer {
    let mut incs = 0;
    let mut last = i64::MAX;

    for win in input.lines().collect::<Vec<&str>>().windows(3) {
        let sum: i64 = win.iter().map(to_int).sum();

        if sum > last {
            incs += 1;
        }

        last = sum;
    }

    incs
}

fn to_int(line: &&str) -> i64 {
    line.parse::<i64>().unwrap()
}
