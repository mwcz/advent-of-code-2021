#![feature(stdin_forwarders)]
use std::io;

#[derive(Debug)]
struct Orientation {
    depth: i64,
    horizontal: i64,
    aim: i64,
}

fn main() {
    let lines = io::stdin().lines();

    let mut pos = Orientation {
        depth: 0,
        horizontal: 0,
        aim: 0,
    };

    for line in lines {
        let cmd_str = line.unwrap();
        let cmd: Vec<&str> = cmd_str.split(' ').collect();

        let axis = cmd.get(0).unwrap();
        let mag = cmd.get(1).unwrap().parse::<i64>().unwrap();

        match *axis {
            "forward" => {
                pos.horizontal += mag;
                pos.depth += pos.aim * mag;
            }
            "up" => pos.aim -= mag,
            "down" => pos.aim += mag,
            _ => {}
        }
    }

    println!("pos: {:?}", pos);
    println!("depth * horizontal = {}", pos.horizontal * pos.depth);
}
