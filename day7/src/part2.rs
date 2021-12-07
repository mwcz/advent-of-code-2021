use std::io;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).unwrap();

    let mut positions: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();

    println!("crab positions: {:?}", positions);

    let mean: i64 = positions.iter().sum::<i64>() / (positions.len() as i64);

    println!("mean: {}", mean);

    let fuel: i64 = positions.iter().map(|pos| {
        let d = (pos - mean).abs();
        (d.pow(2)+d) / 2
    }).sum();

    println!("fuel: {}", fuel);
}
