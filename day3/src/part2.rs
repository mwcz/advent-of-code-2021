#![feature(stdin_forwarders)]
use std::io;

fn bit_hist(bits: &Vec<i64>, tie: i64) -> [i64; 12] {
    let mut bit_hist = [0; 12];

    for line in bits.iter() {
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

    let popular_bits = bit_hist.map(|b: i64| if b > 0  { ((b.abs() / b) + 1) / 2 } else { tie });
    
    popular_bits
}

fn main() {
    let lines: Vec<i64> = io::stdin()
        .lines()
        .map(|bits| i64::from_str_radix(&bits.as_ref().unwrap(), 2).unwrap())
        .collect();

    let popular_bits = bit_hist(&lines, 1);

    let parts = lines
        .iter()
        .partition(|bits| ((*bits & (0b100000000000)) >> 11) == popular_bits[0]);

    let mut ox: Vec<i64> = parts.0;
    let mut co2: Vec<i64> = parts.1;

    for i in (0..11).rev() {

        let mask = 0b000000000001 << i;

        if ox.len() > 1 {
            let popular_bits = bit_hist(&ox, 1);
            ox = ox
                .iter()
                .filter(|bits| ((*bits & mask) >> i) == popular_bits[11-i])
                .map(|bits| *bits)
                .collect();
        }

        if co2.len() > 1 {
            let popular_bits = bit_hist(&co2, 0);
            println!("co2 pop bits: {:?}", popular_bits);
            println!("  i: {}", i);
            println!("  bit mask:    {:#014b}", mask);
            
            co2 = co2
                .iter()
                .filter(|bits| {
                    println!("  pop bit:     {}", popular_bits[11-i]);
                    println!("  bits:        {:#014b}", bits);
                    println!("  filter pass: {}", ((*bits & mask) >> i) != popular_bits[11-i]);
                    
                    ((*bits & mask) >> i) != popular_bits[11-i]
                })
                .map(|bits| *bits)
                .collect();
        }

        println!("ox: {:?}\tco2: {}", ox.len(), co2.len());

    }

    println!("ox: {}", ox.get(0).unwrap());
    println!("co2: {}", co2.get(0).unwrap());
    println!("ox * co2: {}", ox.get(0).unwrap() * co2.get(0).unwrap());
}
