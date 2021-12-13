#![feature(stdin_forwarders)]
use std::io;

const WALL: u8 = 10;

fn main() {
    let lines = io::stdin().lines();

    let mut risks: Vec<u8> = Vec::new();

    let rows = lines
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    for y in 0..rows.len() {
        let row = &rows[y];
        let row_up = rows.get(y.wrapping_sub(1));
        let row_down = rows.get(y.wrapping_add(1));

        for x in 0..rows.get(0).unwrap().len() {
            let h_left = row.get(x.wrapping_sub(1)).unwrap_or(&WALL);
            let h_right = row.get(x.wrapping_add(1)).unwrap_or(&WALL);

            let h = row[x];

            let h_up = match row_up {
                Some(row) => row.get(x).unwrap(),
                None => &WALL,
            };

            let h_down = match row_down {
                Some(row) => row.get(x).unwrap(),
                None => &WALL,
            };

            if h < *h_up && h < *h_down && h < *h_left && h < *h_right {
                // low point found
                risks.push(h + 1);
            }
        }
    }

    println!("risk sum: {}", risks.iter().map(|r| *r as u32).sum::<u32>());
}
