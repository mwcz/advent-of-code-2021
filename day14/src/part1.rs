#![feature(stdin_forwarders, linked_list_cursors, slice_group_by)]
use std::{
    collections::{linked_list::CursorMut, BTreeMap, HashMap, LinkedList},
    io,
};

/// A pair insertion rule.
#[derive(Debug)]
struct PairRule {
    /// The element sequence that matches this pair insertion rule.
    seq: (String, String),
    /// Element letter to insert between the matched sequence.
    sub: String,
}

fn main() {
    let mut lines = io::stdin().lines();

    // parse the numbers on the first line
    let mut polymers: Vec<String> = lines
        .next() // get the first line
        .unwrap() // unwrap Option
        .unwrap() // unwrap Result
        .split("")
        .filter(|p| p.len() > 0)
        .map(|p| p.to_string()) // parse to u32
        .collect();

    // skip the guaranteed empty line
    lines.next();

    // // parse the pair insertion rules
    let rules: Vec<PairRule> = lines
        .map(|r| r.unwrap()) // unwrap Result
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

    println!("Template: {:?}", polymers);

    for step in 0..10 {
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
                return None;
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

    let mut poly_counts: Vec<&i32> = poly_groups.values().collect();
    poly_counts.sort();

    println!("final polymer chain: {:?}", polymers.join(""));
    println!("polymer groups: {:?}", poly_groups);
    println!("polymer counts: {:?}", poly_counts);
    println!(
        "polymer count diff: {:?}",
        **poly_counts.last().unwrap() - **poly_counts.first().unwrap()
    );

    // let mut matches: Vec<(usize, &String)> = rules
    //     .iter()
    //     .map(|rule| {
    //         println!("  applying: {:?}", rule);

    //         let rule_match = polymers.iter().enumerate().find(|(i, p)| {
    //             let next_p = polymers.get(i + 1);
    //             match next_p {
    //                 Some(np) => **p == rule.seq.0 && *np == rule.seq.1,
    //                 None => false,
    //             }
    //         });

    //         rule_match
    //     })
    //     .filter(|m| match m {
    //         Some(_) => true,
    //         None => false,
    //     })
    //     .map(|m| m.unwrap())
    //     .collect();

    // matches.sort_by(|a, b| a.0.cmp(&b.0));

    // let mut poly = polymers.clone();

    // matches
    //     .iter_mut()
    //     .enumerate()
    //     .map(|(i, m)| {
    //         m.0 += i;
    //         *m
    //     })
    //     .for_each(|m| {
    //         poly.insert(m.0, m.1.to_string());
    //     });

    // println!("matches: {:?}", matches);
    // println!("polymers: {:?}", poly);
}
