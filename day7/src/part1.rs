use std::io;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).unwrap();

    let mut positions: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();

    positions.sort();

    let median = positions.get((positions.len() / 2) as usize).unwrap();

    println!("median: {}", median);

    let fuel: i64 = positions.iter().map(|pos| (pos - median).abs()).sum();

    println!("fuel: {}", fuel);
}
