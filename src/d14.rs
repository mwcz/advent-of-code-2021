//! A solution to day 14 year 2021.
//! https://adventofcode.com/2021/day/14

use std::{collections::HashMap, io};

type Model = (ModelPart1, ModelPart2);
type Answer = i64;
type ModelPart1 = (Vec<String>, Vec<PairRule>);
type ModelPart2 = (Vec<String>, Vec<(usize, usize)>, Vec<String>);

pub fn parse(input: String) -> Model {
    (parse_part1(input.clone()), parse_part2(input.clone()))
}

pub fn parse_part1(input: String) -> (Vec<String>, Vec<PairRule>) {
    let mut lines = input.lines();
    // parse the numbers on the first line
    let mut polymers: Vec<String> = lines
        .next() // get the first line
        .unwrap() // unwrap Option
        .split("")
        .filter(|p| !p.is_empty())
        .map(|p| p.to_string()) // parse to u32
        .collect();

    // skip the guaranteed empty line
    lines.next();

    // // parse the pair insertion rules
    let rules: Vec<PairRule> = lines
        // .collect::<Vec<String>>() // collect into a Vec of Strings, couldn't figure out how to split a std::io::Lines
        // .split(|line| line.len() == 0) // split on empty lines
        .map(|line| {
            let seq: Vec<String> = line
                .trim()
                .to_string()
                .split(" -> ")
                .map(|s| s.to_string())
                .collect();
            PairRule {
                seq: (
                    seq[0].get(0..1).unwrap().to_string().clone(),
                    seq[0].get(1..2).unwrap().to_string().clone(),
                ),
                sub: seq[1].clone(),
            }
        })
        .collect();

    (polymers, rules)
}

pub fn parse_part2(input: String) -> ModelPart2 {
    let mut lines = input.lines();

    // parse the numbers on the first line
    let polymers: Vec<String> = lines
        .next() // get the first line
        .unwrap() // unwrap Option
        .split("")
        .filter(|p| !p.is_empty())
        .map(|p| p.to_string()) // parse to u32
        .collect();

    // skip the guaranteed empty line
    lines.next();

    // first make a data structure of the form:
    //  [("CH", "CB", "BH"),
    //   ("HH", "HN", "NH"),
    //   ("CB", "CH", "HB")]
    // The first item in the vec means "CH -> CB,BH"
    let rules: Vec<(String, String, String)> = lines
        .map(|line| {
            let seq: Vec<String> = line
                .trim()
                .to_string()
                .split(" -> ")
                .map(|s| s.to_string())
                .collect();
            let letter_a = seq[0].get(0..1).unwrap().to_string();
            let letter_b = seq[0].get(1..2).unwrap().to_string();
            let new_letter = &seq[1];
            let pair_a = letter_a.clone() + &new_letter.clone();
            let pair_b = new_letter.clone() + &letter_b.clone();

            (seq[0].clone(), pair_a, pair_b)
            // polymers.windows(2).find
        })
        .collect();

    // get a data structure containing all the possible pairs
    let pairs: Vec<String> = rules.iter().map(|rule| rule.0.clone()).collect();

    let rules: Vec<(usize, usize)> = rules
        .iter()
        .map(|rule| {
            let index_a = rules.iter().position(|r| r.0 == rule.1).unwrap();
            let index_b = rules.iter().position(|r| r.0 == rule.2).unwrap();
            (index_a, index_b)
        })
        .collect();

    (polymers, rules, pairs)
}

pub fn part1(((mut polymers, rules), _): Model) -> Answer {
    // println!("Template: {:?}", polymers);

    for _step in 0..10 {
        let insertions: Vec<(usize, String)> = polymers
            .windows(2)
            .enumerate()
            .map(|(i, pair)| {
                let a = &pair[0][0..1];
                let b = &pair[1][0..1];
                for rule in &rules {
                    if a == rule.seq.0 && b == rule.seq.1 {
                        return Some((i + 1, rule.sub.clone()));
                    }
                }
                None
            })
            .enumerate()
            .map(|(i, ins)| {
                // progressively inc the insertions points so they wind up in the right spot as the polymers vec grows
                let mut ins = ins.unwrap();
                ins.0 += i;
                ins
            })
            .collect();

        // do the insertions
        insertions.iter().for_each(|(i, ins)| {
            // println!("  inserting {} at index {}", ins, i);
            polymers.insert(*i, ins.to_string());
        });

        // println!("After step {}: {:?}", step, insertions);
    }

    // count up how many of each letter there are
    let poly_groups = polymers.iter().fold(HashMap::new(), |mut hash, polymer| {
        let counter = hash.entry(polymer).or_insert(1);
        *counter += 1;
        hash
    });

    let mut poly_counts: Vec<&i64> = poly_groups.values().collect();
    poly_counts.sort();

    // println!("final polymer chain: {:?}", polymers.join(""));
    // println!("polymer groups: {:?}", poly_groups);
    // println!("polymer counts: {:?}", poly_counts);
    // println!(
    //     "polymer count diff: {:?}",
    //     **poly_counts.last().unwrap() - **poly_counts.first().unwrap()
    // );
    **poly_counts.last().unwrap() - **poly_counts.first().unwrap()
}

pub fn part2((_, (mut polymers, rules, pairs)): Model) -> Answer {
    let mut pair_counts = vec![0_i64; pairs.len()];
    let mut step_count = vec![0_i64; pairs.len()];

    // count the initial pairs
    polymers.windows(2).for_each(|pair| {
        let initial_pair = pair.first().unwrap().to_string() + &pair.last().unwrap().to_string();
        let initial_pair_i = pairs.iter().position(|p| *p == initial_pair).unwrap();
        pair_counts[initial_pair_i] += 1;
    });

    for _steps in 0..STEPS {
        step_count.fill(0);
        for (i, lc) in pair_counts.iter().enumerate() {
            step_count[rules[i].0] += *lc;
            step_count[rules[i].1] += *lc;
        }
        for (i, count) in pair_counts.iter_mut().enumerate() {
            *count = step_count[i];
        }
    }

    // println!("Pair counts:");
    // for (i, count) in pair_counts.iter().enumerate() {
    //     println!("  {}: {}", pairs[i], count);
    // }

    let mut letters_indexed: Vec<(usize, &String)> =
        pairs.iter().enumerate().collect::<Vec<(usize, &String)>>();

    letters_indexed.sort_by(|a, b| a.1[0..1].cmp(&b.1[0..1]));

    let mut letter_counts: Vec<(String, i64)> = letters_indexed
        .group_by(|a, b| a.1[0..1] == b.1[0..1])
        .map(|group| {
            let last_letter_bonus = if group[0].1[0..1] == *polymers.last().unwrap() {
                1
            } else {
                0
            };
            let group_count: i64 = group.iter().map(|(i, _pair)| pair_counts[*i]).sum();
            (
                group[0].1[0..1].to_string(),
                group_count + last_letter_bonus,
            )
        })
        .collect();

    letter_counts.sort_by(|(_, a), (_, b)| a.cmp(b));

    // println!("Letter counts:");
    // for (letter, count) in &letter_counts {
    //     println!("  {} count: {}", letter, count);
    // }

    // println!(
    //     "Answer: {}",
    //     letter_counts.last().unwrap().1 - letter_counts.first().unwrap().1
    // );

    let most = letter_counts.last().unwrap().1;
    let least = letter_counts.first().unwrap().1;

    most - least
}

/// A pair insertion rule.
#[derive(Debug)]
pub struct PairRule {
    /// The element sequence that matches this pair insertion rule.
    seq: (String, String),
    /// Element letter to insert between the matched sequence.
    sub: String,
}

const STEPS: u32 = 40;
