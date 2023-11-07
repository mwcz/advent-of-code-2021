//! A solution to day 3 year 2021.
//! https://adventofcode.com/2021/day/3

use std::cmp::Ordering::{Equal, Greater, Less};

type Model = String;
type Answer = u64;

pub fn parse(input: String) -> Model {
    input
}

pub fn part1(input: Model) -> Answer {
    let lines = input.lines();

    let mut bit_hist = [0; 12];

    for bits in lines {
        let chars = bits.chars();

        // 0 gives -1, 1 gives +1, so in the end, positive indicates more 1's, and negative more
        //   0's
        for (i, bit) in chars.enumerate() {
            match bit {
                '0' => bit_hist[i] -= 1,
                '1' => bit_hist[i] += 1,
                _ => {} // ignore other chars
            }
        }
    }

    // println!("bit_hist {:?}", bit_hist);

    let bit_string = bit_hist
        .map(|b: i64| ((b.abs() / b) + 1) / 2)
        .map(|b: i64| b as u64)
        .map(|b: u64| b.to_string())
        .join("");

    let gamma = u64::from_str_radix(&bit_string, 2).unwrap();
    let epsilon = gamma ^ 0b111111111111; // flip the lower 12 bits

    // println!("gamma: {}", gamma);
    // println!("epsilon: {}", epsilon);
    // println!("gamma * epsilon: {}", epsilon * gamma);
    epsilon * gamma
}

pub fn part2(input: Model) -> i64 {
    const WIDTH: usize = 12;

    fn bit_hist(bits: &[i64], index: usize, tie: i64) -> i64 {
        let mut rank = 0;

        // add 1 for each 1, sub 1 for each 0
        for line in bits.iter() {
            rank += -1 + 2 * ((line & (0b000000000001 << index)) >> index);
        }

        match rank.cmp(&0) {
            Greater => 1,
            Less => 0,
            Equal => tie,
        }
    }

    let lines: Vec<i64> = input
        .lines()
        .map(|bits| i64::from_str_radix(bits, 2).unwrap())
        .collect();

    // partition the initial data set into those that start with 1 and those that start with 0
    // (could do two filters, but this way avoids cloning).
    let pop_bit = bit_hist(&lines, WIDTH - 1, 1);
    let parts = lines
        .iter()
        .partition(|bits| ((*bits & (0b100000000000)) >> (WIDTH - 1)) == pop_bit);

    let mut ox: Vec<i64> = parts.0;
    let mut co2: Vec<i64> = parts.1;

    for i in (0..(WIDTH - 1)).rev() {
        let mask = 0b000000000001 << i;

        if ox.len() > 1 {
            let pop_bit = bit_hist(&ox, i, 1);
            ox.retain(|bits| ((*bits & mask) >> i) == pop_bit);
        }

        if co2.len() > 1 {
            let pop_bit = bit_hist(&co2, i, 1);

            co2.retain(|bits| ((*bits & mask) >> i) != pop_bit);
        }
    }

    // println!("ox: {}", ox.first().unwrap());
    // println!("co2: {}", co2.first().unwrap());
    // println!("ox * co2: {}", ox.first().unwrap() * co2.first().unwrap());

    ox.first().unwrap() * co2.first().unwrap()
}
