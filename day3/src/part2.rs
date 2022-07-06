use std::io;

const WIDTH: usize = 12;

fn bit_hist(bits: &Vec<i64>, index: usize, tie: i64) -> i64 {
    let mut rank = 0;

    // add 1 for each 1, sub 1 for each 0
    for line in bits.iter() {
        rank += -1 + 2 * ((line & (0b000000000001 << index)) >> index);
    }

    if rank > 0 {
        1
    } else if rank < 0 {
        0
    } else {
        tie
    }
}

fn main() {
    let lines: Vec<i64> = io::stdin()
        .lines()
        .map(|bits| i64::from_str_radix(&bits.as_ref().unwrap(), 2).unwrap())
        .collect();

    // partition the initial data set into those that start with 1 and those that start with 0
    // (could do two filters, but this way avoids cloning).
    let pop_bit = bit_hist(&lines, WIDTH - 1, 1);
    let parts = lines
        .iter()
        .partition(|bits| ((*bits & (0b100000000000)) >> (WIDTH - 1)) == pop_bit);

    let mut ox: Vec<i64> = parts.0;
    let mut co2: Vec<i64> = parts.1;

    for i in (0..(WIDTH - 1)).rev() {
        let mask = 0b000000000001 << i;

        if ox.len() > 1 {
            let pop_bit = bit_hist(&ox, i, 1);
            ox = ox
                .iter()
                .filter(|bits| ((*bits & mask) >> i) == pop_bit)
                .map(|bits| *bits)
                .collect();
        }

        if co2.len() > 1 {
            let pop_bit = bit_hist(&co2, i, 1);

            co2 = co2
                .iter()
                .filter(|bits| ((*bits & mask) >> i) != pop_bit)
                .map(|bits| *bits)
                .collect();
        }
    }

    println!("ox: {}", ox.get(0).unwrap());
    println!("co2: {}", co2.get(0).unwrap());
    println!("ox * co2: {}", ox.get(0).unwrap() * co2.get(0).unwrap());
}
