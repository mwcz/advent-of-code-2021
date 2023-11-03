//! A solution to day 6 year 2021.
//! https://adventofcode.com/2021/day/6

use crate::answer::Answer;

type Parsed = [i64; 9];

pub fn parse(input: String) -> Parsed {
    let numbers: Vec<i64> = input
        .split(',')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    let mut counts: [i64; 9] = [0; 9];

    // seed initial counts

    for n in &numbers {
        counts[*n as usize] += 1;
    }

    counts
}

pub fn part1(counts: Parsed) -> impl Answer {
    solve(counts, 80)
}

pub fn part2(counts: Parsed) -> impl Answer {
    solve(counts, 256)
}

pub fn solve(mut counts: Parsed, days: usize) -> impl Answer {
    // simulate days
    for _ in 0..days {
        let spawns = counts[0];

        counts[0] = counts[1];
        counts[1] = counts[2];
        counts[2] = counts[3];
        counts[3] = counts[4];
        counts[4] = counts[5];
        counts[5] = counts[6];
        counts[6] = counts[7] + spawns;
        counts[7] = counts[8];
        counts[8] = spawns;
    }

    counts.iter().sum::<i64>()
}
