#![feature(stdin_forwarders)]
use std::io;

fn main() {
    let lines = io::stdin().lines();

    let mut incs = 0;
    let mut last = i64::MAX;

    for line in lines {
        let number = line.unwrap().trim_end().parse::<i64>().unwrap();

        if number > last {
            incs += 1;
        }

        last = number;
    }

    println!("Increments: {}", incs);
}
