use std::io;

fn main() {
    let lines = io::stdin().lines();

    let digits: usize = lines
        .map(|line| {
            line.unwrap()
                .split('|')
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|digit| digit.to_string())
                .inspect(|d| {
                    dbg!(d);
                    return ();
                })
                .filter(|digit| [2, 3, 4, 7].contains(&digit.len()))
                .collect::<Vec<_>>()
                .len()
        })
        .sum();

    println!("perfect digits: {}", digits);
}
