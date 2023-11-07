//! A solution to day 2 year 2021.
//! https://adventofcode.com/2021/day/2

type Model = String;
type Answer = i64;

pub fn parse(input: String) -> Model {
    input
}

pub fn part1(input: Model) -> Answer {
    // println!("pos: {:?}", pos);
    // println!("depth * horizontal = {}", pos.horizontal * pos.depth);
    let mut pos = Orientation {
        depth: 0,
        horizontal: 0,
        aim: 0,
    };

    for line in input.lines() {
        let cmd: Vec<&str> = line.split(' ').collect();

        let axis = cmd.get(0).unwrap();
        let mag = cmd.get(1).unwrap().parse::<i64>().unwrap();

        match *axis {
            "forward" => {
                pos.horizontal += mag;
                pos.depth += pos.aim * mag;
            }
            "up" => pos.depth -= mag,
            "down" => pos.depth += mag,
            _ => {}
        }
    }
    pos.horizontal * pos.depth
}

pub fn part2(input: Model) -> Answer {
    let mut pos = Orientation {
        depth: 0,
        horizontal: 0,
        aim: 0,
    };

    for line in input.lines() {
        let cmd: Vec<&str> = line.split(' ').collect();

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
    pos.horizontal * pos.depth
}

#[derive(Debug)]
struct Orientation {
    depth: i64,
    horizontal: i64,
    aim: i64,
}
