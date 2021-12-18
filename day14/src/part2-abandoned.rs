// #![feature(stdin_forwarders, linked_list_cursors, slice_group_by)]
// use std::{
//     collections::{HashMap, LinkedList},
//     io,
//     iter::Map,
// };

// /// A pair insertion rule.
// #[derive(Debug)]
// struct PairRule {
//     /// The element sequence that matches this pair insertion rule.
//     seq: (String, String),
//     /// Element letter to insert between the matched sequence.
//     sub: String,
// }

// #[derive(Debug)]
// enum Pairs {
//     CH,
//     HH,
//     CB,
//     NH,
//     HB,
//     HC,
//     HN,
//     NN,
//     BH,
//     NC,
//     NB,
//     BN,
//     BB,
//     BC,
//     CC,
//     CN,
// }

// impl Pairs {
//     fn lookup(pair: &[&str]) -> Option<Self> {
//         match pair {
//             ["C", "H"] => Some(Self::CH),
//             ["H", "H"] => Some(Self::HH),
//             ["C", "B"] => Some(Self::CB),
//             ["N", "H"] => Some(Self::NH),
//             ["H", "B"] => Some(Self::HB),
//             ["H", "C"] => Some(Self::HC),
//             ["H", "N"] => Some(Self::HN),
//             ["N", "N"] => Some(Self::NN),
//             ["B", "H"] => Some(Self::BH),
//             ["N", "C"] => Some(Self::NC),
//             ["N", "B"] => Some(Self::NB),
//             ["B", "N"] => Some(Self::BN),
//             ["B", "B"] => Some(Self::BB),
//             ["B", "C"] => Some(Self::BC),
//             ["C", "C"] => Some(Self::CC),
//             ["C", "N"] => Some(Self::CN),
//             _ => None,
//         }
//     }

//     fn lookahead(&self, steps: u32) -> &str {
//         match self {
//             Self::CH => {
//                 if steps % 2 == 0 {
//                     "H"
//                 } else {
//                     "B"
//                 }
//             }
//             Self::HH => {
//                 if steps == 1 {
//                     "N"
//                 } else {
//                     Pairs::HC.lookahead(steps)
//                 }
//             }
//             Self::CB => Pairs::CH.lookahead(steps),
//             Self::NH => {
//                 if steps == 1 {
//                     "C"
//                 } else {
//                     "B"
//                 }
//             }
//             Self::HB => {
//                 if steps % 2 == 0 {
//                     "B"
//                 } else {
//                     "C"
//                 }
//             }
//             Self::HC => Pairs::HB.lookahead(steps + 1),
//             Self::HN => Pairs::HB.lookahead(steps),
//             Self::NN => {
//                 if steps == 1 {
//                     "C"
//                 } else {
//                     "B"
//                 }
//             }
//             Self::BH => "H",
//             Self::NC => "B",
//             Self::NB => "B",
//             Self::BN => {
//                 if steps % 2 == 0 {
//                     "N"
//                 } else {
//                     "B"
//                 }
//             }
//             Self::BB => Pairs::BN.lookahead(steps + 1),
//             Self::BC => Pairs::BN.lookahead(steps),
//             Self::CC => {
//                 if steps % 2 == 0 {
//                     "C"
//                 } else {
//                     "N"
//                 }
//             }
//             Self::CN => Pairs::CC.lookahead(steps + 1),
//         }
//     }
// }

// fn polymer_len(steps: u64, initial_len: u64) -> u64 {
//     if steps == 0 {
//         return initial_len;
//     } else {
//         return 2 * polymer_len(steps - 1, initial_len) - 1;
//     }
// }

// fn main() {
//     let mut lines = io::stdin().lines();

//     println!("length 20 polymer will be this long after...");
//     println!("40 steps -> {}", polymer_len(40, 20));
//     println!("10 steps -> {}", polymer_len(10, 20));
//     println!(" 4 steps -> {}", polymer_len(4, 20));

//     let template = ["N", "N", "C", "B"];
//     let steps = 4;

//     let mut counts = HashMap::new();
//     counts.insert("H", 0);
//     counts.insert("N", 0);
//     counts.insert("C", 0);
//     counts.insert("B", 0);

//     for letter_pair in template.windows(2) {
//         let mut letter_a = letter_pair[0];
//         let mut letter_b = letter_pair[1];

//         println!("{}{}", letter_a, letter_b);

//         for step in (1..steps + 1).rev() {
//             let pair = Pairs::lookup(&[letter_a, letter_b]).unwrap();
//             println!("  step {}", step);

//             for substep in (1..step + 1).rev() {
//                 println!("    {}.{}", step, substep);
//             }
//         }
//     }

//     // println!(
//     //     "{}{} -> {}{}{}",
//     //     letters[0],
//     //     letters[1],
//     //     letters[0],
//     //     pair.lookahead(4),
//     //     letters[1]
//     // );

//     // // parse the numbers on the first line
//     // let mut polymers: LinkedList<String> = lines
//     //     .next() // get the first line
//     //     .unwrap() // unwrap Option
//     //     .unwrap() // unwrap Result
//     //     .split("")
//     //     .filter(|p| p.len() > 0)
//     //     .map(|p| p.to_string()) // parse to u32
//     //     .collect();

//     // // skip the guaranteed empty line
//     // lines.next();

//     // // // parse the pair insertion rules
//     // let rules: Vec<PairRule> = lines
//     //     .map(|r| r.unwrap()) // unwrap Result
//     //     // .collect::<Vec<String>>() // collect into a Vec of Strings, couldn't figure out how to split a std::io::Lines
//     //     // .split(|line| line.len() == 0) // split on empty lines
//     //     .map(|line| {
//     //         let seq: Vec<String> = line
//     //             .trim()
//     //             .to_string()
//     //             .split(" -> ")
//     //             .map(|s| s.to_string())
//     //             .collect();
//     //         PairRule {
//     //             seq: (
//     //                 seq[0].get(0..1).unwrap().to_string().clone(),
//     //                 seq[0].get(1..2).unwrap().to_string().clone(),
//     //             ),
//     //             sub: seq[1].clone(),
//     //         }
//     //     })
//     //     .collect();

//     // println!("Template: {:?}", polymers);

//     // for step in 0..40 {
//     //     println!("starting step {}", step);
//     //     let insertions: Vec<(usize, &String)> = polymers
//     //         .iter()
//     //         .enumerate()
//     //         .map(|(i, a)| {
//     //             // let a = [0..1];
//     //             let b = a;
//     //             for rule in &rules {
//     //                 if a == &rule.seq.0 && b == &rule.seq.1 {
//     //                     return Some((i + 1, &rule.sub));
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
//     //         let mut tail = polymers.split_off(*i);
//     //         polymers.push_back(ins.to_string());
//     //         polymers.append(&mut tail);
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

//     // // println!("final polymer chain: {:?}", polymers.join(""));
//     // println!("polymer groups: {:?}", poly_groups);
//     // println!("polymer counts: {:?}", poly_counts);
//     // println!(
//     //     "polymer count diff: {:?}",
//     //     **poly_counts.last().unwrap() - **poly_counts.first().unwrap()
//     // );
// }
