#![feature(stdin_forwarders)]
use std::io::{self, Error};

fn main() {
    let lines: Vec<i64> = io::stdin()
        .lines()
        .map(|bits| i64::from_str_radix(&bits.as_ref().unwrap(), 2).unwrap())
        .collect();

    let mut bit_hist = [0; 12];

    for line in &lines {
        bit_hist[0] += -1 + 2 * ((line & 0b100000000000) >> 11);
        bit_hist[1] += -1 + 2 * ((line & 0b010000000000) >> 10);
        bit_hist[2] += -1 + 2 * ((line & 0b001000000000) >> 9);
        bit_hist[3] += -1 + 2 * ((line & 0b000100000000) >> 8);
        bit_hist[4] += -1 + 2 * ((line & 0b000010000000) >> 7);
        bit_hist[5] += -1 + 2 * ((line & 0b000001000000) >> 6);
        bit_hist[6] += -1 + 2 * ((line & 0b000000100000) >> 5);
        bit_hist[7] += -1 + 2 * ((line & 0b000000010000) >> 4);
        bit_hist[8] += -1 + 2 * ((line & 0b000000001000) >> 3);
        bit_hist[9 ] += -1 + 2 * ((line & 0b000000000100) >> 2);
        bit_hist[10] += -1 + 2 * ((line & 0b000000000010) >> 1);
        bit_hist[11] += -1 + 2 * (line & 0b000000000001);
    }

    let popular_bits = bit_hist.map(|b: i64| ((b.abs() / b) + 1) / 2);

    let bit_string = popular_bits
        .map(|b: i64| b as u64)
        .map(|b: u64| b.to_string())
        .join("");

    println!("bit_hist {:?}", bit_hist);
    println!("bit_string {}", bit_string);

    let mut lines: Vec<i64> = lines;

    for i in (0..12).rev() {

        lines = lines
            .iter()
            .filter(|bits| ((*bits & (0b000000000001 << i)) >> i) == popular_bits[i])
            .map(|bits| *bits)
            .collect();

        if lines.len() == 1 {
            break;
        }

    }

    println!("lines {:?}", lines);

    let gamma = u64::from_str_radix(&bit_string, 2).unwrap();
    let epsilon = gamma ^ 0b111111111111; // flip the lower 12 bits

    println!("gamma: {}", gamma);
    println!("epsilon: {}", epsilon);
    println!("gamma * epsilon: {}", epsilon * gamma);
}
