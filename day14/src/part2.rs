use std::io;

const STEPS: u32 = 40;

fn main() {
    let mut lines = io::stdin().lines();

    // parse the numbers on the first line
    let polymers: Vec<String> = lines
        .next() // get the first line
        .unwrap() // unwrap Option
        .unwrap() // unwrap Result
        .split("")
        .filter(|p| p.len() > 0)
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
        .map(|r| r.unwrap())
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

    let mut pair_counts = vec![0 as i64; pairs.len()];
    let mut step_count = vec![0 as i64; pairs.len()];

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

    println!("Pair counts:");
    for (i, count) in pair_counts.iter().enumerate() {
        println!("  {}: {}", pairs[i], count);
    }

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

    letter_counts.sort();

    println!("Letter counts:");
    for (letter, count) in &letter_counts {
        println!("  {} count: {}", letter, count);
    }

    println!(
        "Answer: {}",
        letter_counts.last().unwrap().1 - letter_counts.first().unwrap().1
    );
}
