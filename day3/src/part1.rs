#![feature(stdin_forwarders)]
use std::io;

fn main() {
    let lines = io::stdin().lines();

    let mut bit_hist = [0; 12];

    for line in lines {
        let bits = line.unwrap();
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

    println!("bit_hist {:?}", bit_hist);

    let bit_string = bit_hist
        .map(|b: i64| ((b.abs() / b) + 1) / 2)
        .map(|b: i64| b as u64)
        .map(|b: u64| b.to_string())
        .join("");

    let gamma = u64::from_str_radix(&bit_string, 2).unwrap();
    let epsilon = gamma ^ 0b111111111111; // flip the lower 12 bits

    println!("gamma: {}", gamma);
    println!("epsilon: {}", epsilon);
    println!("gamma * epsilon: {}", epsilon * gamma);
}
