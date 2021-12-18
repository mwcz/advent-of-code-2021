// #![feature(stdin_forwarders, linked_list_cursors, slice_group_by)]
// use std::{collections::HashMap, io};

// const STEPS: u32 = 10;

// type Pair = (String, String);

// /// A pair insertion rule.
// struct PairRule {
//     /// The element sequence that matches this pair insertion rule.
//     seq: Pair,
//     /// Element letter to insert between the matched sequence.
//     sub: (Pair, Pair),
// }

// fn letter_count<'a>(
//     pair: &Pair,
//     max_steps: u32,
//     step: u32,
//     rules: &'a HashMap<Pair, (Pair, Pair)>,
//     acc: &'a mut HashMap<String, u64>,
//     total_len: u64,
// ) -> &'a HashMap<String, u64> {
//     // if we're at the final step, count up the left letter in the pair
//     if step == max_steps {
//         let counter = acc.entry(pair.0.clone()).or_insert(0);
//         *counter += 1;

//         let current_count = acc.values().sum::<u64>();
//         if current_count % 10_000_000 == 0 {
//             println!(
//                 "progress ({}) {} / {}",
//                 (current_count as f64) / (total_len as f64),
//                 current_count,
//                 total_len
//             );
//         }
//     }

//     // if we have at least one more step to go, keep chugging
//     if step <= max_steps {
//         let new_pairs = rules.get(&pair).unwrap();
//         letter_count(&new_pairs.0, max_steps, step + 1, rules, acc, total_len);
//         letter_count(&new_pairs.1, max_steps, step + 1, rules, acc, total_len);
//     }

//     // finally, return the map storing the letter counts
//     acc
// }

// fn polymer_len(steps: u32, initial_len: u32) -> u64 {
//     if steps == 0 {
//         return initial_len.into();
//     } else {
//         return 2 * polymer_len(steps - 1, initial_len) - 1;
//     }
// }

// fn main() {
//     let mut lines = io::stdin().lines();

//     // parse the numbers on the first line
//     let polymers: Vec<String> = lines
//         .next() // get the first line
//         .unwrap() // unwrap Option
//         .unwrap() // unwrap Result
//         .split("")
//         .filter(|p| p.len() > 0)
//         .map(|p| p.to_string()) // parse to u32
//         .collect();

//     // skip the guaranteed empty line
//     lines.next();

//     // // parse the pair insertion rules
//     let rules: HashMap<Pair, (Pair, Pair)> = lines
//         .map(|r| r.unwrap()) // unwrap Result
//         // .collect::<Vec<String>>() // collect into a Vec of Strings, couldn't figure out how to split a std::io::Lines
//         // .split(|line| line.len() == 0) // split on empty lines
//         .map(|line| {
//             let seq: Vec<String> = line
//                 .trim()
//                 .to_string()
//                 .split(" -> ")
//                 .map(|s| s.to_string())
//                 .collect();
//             let letter_a = seq[0].get(0..1).unwrap().to_string();
//             let letter_b = seq[0].get(1..2).unwrap().to_string();
//             let new_letter = &seq[1];
//             PairRule {
//                 seq: (letter_a.clone(), letter_b.clone()),
//                 sub: (
//                     (letter_a.clone(), new_letter.clone()),
//                     (new_letter.clone(), letter_b.clone()),
//                 ),
//             }
//         })
//         .fold(HashMap::new(), |mut map, rule| {
//             map.insert(rule.seq, rule.sub);
//             map
//         });

//     println!("Template: {:?}", polymers);

//     // println!("Rules:");
//     // for rule in &rules {
//     //     println!("  {:?}", rule);
//     // }

//     let mut letter_counts = HashMap::new();

//     polymers.windows(2).for_each(|pair| {
//         // count the expanding tree of letters
//         letter_count(
//             &(
//                 pair.first().unwrap().to_string(),
//                 pair.last().unwrap().to_string(),
//             ),
//             STEPS,
//             0,
//             &rules,
//             &mut letter_counts,
//             polymer_len(STEPS, polymers.len() as u32),
//         );
//     });
//     // count the final letter, which is ignored by the letter_count algorithm to avoid
//     // double-counting
//     let counter = letter_counts
//         .entry(polymers.last().unwrap().to_string())
//         .or_insert(0);
//     *counter += 1;

//     // NN
//     // NC CN
//     // NBCCN

//     // NNCB
//     // NN NC CB
//     // NCNBCHB

//     let mut counts: Vec<&u64> = letter_counts.values().collect();
//     counts.sort();

//     // println!("final polymer chain: {:?}", polymers.join(""));
//     println!("polymer counts: {:?}", counts);
//     println!(
//         "polymer count diff: {:?}",
//         **counts.last().unwrap() - **counts.first().unwrap()
//     );

//     // for _step in 0..1 {{{{
//     //     let insertions: Vec<(usize, String)> = polymers
//     //         .windows(2)
//     //         .enumerate()
//     //         .map(|(i, pair)| {
//     //             let a = &pair[0][0..1];
//     //             let b = &pair[1][0..1];
//     //             for rule in &rules {
//     //                 if a == rule.seq.0 && b == rule.seq.1 {
//     //                     return Some((i + 1, rule.sub.clone()));
//     //                 }
//     //             }
//     //             return None;
//     //         })
//     //         .enumerate()
//     //         .map(|(i, ins)| {
//     //             // progressively inc the insertions points so they wind up in the right spot as the polymers vec grows
//     //             let mut ins = ins.unwrap();
//     //             ins.0 += i;
//     //             ins
//     //         })
//     //         .collect();

//     //     // do the insertions
//     //     insertions.iter().for_each(|(i, ins)| {
//     //         // println!("  inserting {} at index {}", ins, i);
//     //         polymers.insert(*i, ins.to_string());
//     //     });

//     //     // println!("After step {}: {:?}", step, insertions);
//     // }

//     // // count up how many of each letter there are
//     // let poly_groups = polymers.iter().fold(HashMap::new(), |mut hash, polymer| {
//     //     let counter = hash.entry(polymer).or_insert(1);
//     //     *counter += 1;
//     //     hash
//     // });

//     // let mut poly_counts: Vec<&i32> = poly_groups.values().collect();
//     // poly_counts.sort();

//     // println!("final polymer chain: {:?}", polymers.join(""));
//     // println!("polymer groups: {:?}", poly_groups);
//     // println!("polymer counts: {:?}", poly_counts);
//     // println!(
//     //     "polymer count diff: {:?}",
//     //     **poly_counts.last().unwrap() - **poly_counts.first().unwrap()
//     // );}}}
// }

// // vim: set foldmethod=marker:
