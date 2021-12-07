use std::io;

const DAYS: i32 = 256;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).unwrap();

    let numbers: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();

    let mut counts: [i64; 9] = [0; 9];

    // seed initial counts

    for n in &numbers {
        counts[*n as usize] += 1;
    }

    // simulate days

    for _ in 0..DAYS {
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

    println!(
        "fish population after {} days: {}",
        DAYS,
        counts.iter().sum::<i64>()
    );
}
