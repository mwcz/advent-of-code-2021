use argh::FromArgs;
use std::fs::read_to_string;

fn main() {
    let args: Args = argh::from_env();

    if args.day == 255 {
        for day in 1..=25 {
            let input =
                read_to_string(format!("./input/d{}", day)).expect("couldn't read input file");
            print!("day {day} part 1: ");
            run(day, 1, input.clone());
            print!("day {day} part 2: ");
            run(day, 2, input.clone());
        }
    } else if (1..=25).contains(&args.day) {
        let input_file = if args.input.is_some() {
            args.input.unwrap()
        } else if args.example {
            format!("./examples/d{}", args.day)
        } else {
            format!("./input/d{}", args.day)
        };

        let input = read_to_string(input_file).expect("couldn't read input file");
        run(args.day, args.part, input);
    } else {
        eprintln!("day must be 1 through 25, or 255 to run all days");
    }
}

fn run(day: u8, part: u8, input: String) {
    match (day, part) {
        (1, 1) => {
            let parsed = aoc2021::d1::parse(input);
            let output = aoc2021::d1::part1(parsed);
            println!("{output}");
        }
        (1, 2) => {
            let parsed = aoc2021::d1::parse(input);
            let output = aoc2021::d1::part2(parsed);
            println!("{output}");
        }
        (2, 1) => {
            let parsed = aoc2021::d2::parse(input);
            let output = aoc2021::d2::part1(parsed);
            println!("{output}");
        }
        (2, 2) => {
            let parsed = aoc2021::d2::parse(input);
            let output = aoc2021::d2::part2(parsed);
            println!("{output}");
        }
        (3, 1) => {
            let parsed = aoc2021::d3::parse(input);
            let output = aoc2021::d3::part1(parsed);
            println!("{output}");
        }
        (3, 2) => {
            let parsed = aoc2021::d3::parse(input);
            let output = aoc2021::d3::part2(parsed);
            println!("{output}");
        }
        (4, 1) => {
            let parsed = aoc2021::d4::parse(input);
            let output = aoc2021::d4::part1(parsed);
            println!("{output}");
        }
        (4, 2) => {
            let parsed = aoc2021::d4::parse(input);
            let output = aoc2021::d4::part2(parsed);
            println!("{output}");
        }
        (5, 1) => {
            let parsed = aoc2021::d5::parse(input);
            let output = aoc2021::d5::part1(parsed);
            println!("{output}");
        }
        (5, 2) => {
            let parsed = aoc2021::d5::parse(input);
            let output = aoc2021::d5::part2(parsed);
            println!("{output}");
        }
        (6, 1) => {
            let parsed = aoc2021::d6::parse(input);
            let output = aoc2021::d6::part1(parsed);
            println!("{output}");
        }
        (6, 2) => {
            let parsed = aoc2021::d6::parse(input);
            let output = aoc2021::d6::part2(parsed);
            println!("{output}");
        }
        (7, 1) => {
            let parsed = aoc2021::d7::parse(input);
            let output = aoc2021::d7::part1(parsed);
            println!("{output}");
        }
        (7, 2) => {
            let parsed = aoc2021::d7::parse(input);
            let output = aoc2021::d7::part2(parsed);
            println!("{output}");
        }
        (8, 1) => {
            let parsed = aoc2021::d8::parse(input);
            let output = aoc2021::d8::part1(parsed);
            println!("{output}");
        }
        (8, 2) => {
            let parsed = aoc2021::d8::parse(input);
            let output = aoc2021::d8::part2(parsed);
            println!("{output}");
        }
        (9, 1) => {
            let parsed = aoc2021::d9::parse(input);
            let output = aoc2021::d9::part1(parsed);
            println!("{output}");
        }
        (9, 2) => {
            let parsed = aoc2021::d9::parse(input);
            let output = aoc2021::d9::part2(parsed);
            println!("{output}");
        }
        (10, 1) => {
            let parsed = aoc2021::d10::parse(input);
            let output = aoc2021::d10::part1(parsed);
            println!("{output}");
        }
        (10, 2) => {
            let parsed = aoc2021::d10::parse(input);
            let output = aoc2021::d10::part2(parsed);
            println!("{output}");
        }
        (11, 1) => {
            let parsed = aoc2021::d11::parse(input);
            let output = aoc2021::d11::part1(parsed);
            println!("{output}");
        }
        (11, 2) => {
            let parsed = aoc2021::d11::parse(input);
            let output = aoc2021::d11::part2(parsed);
            println!("{output}");
        }
        (12, 1) => {
            let parsed = aoc2021::d12::parse(input);
            let output = aoc2021::d12::part1(parsed);
            println!("{output}");
        }
        (12, 2) => {
            let parsed = aoc2021::d12::parse(input);
            let output = aoc2021::d12::part2(parsed);
            println!("{output}");
        }
        (13, 1) => {
            let parsed = aoc2021::d13::parse(input);
            let output = aoc2021::d13::part1(parsed);
            println!("{output}");
        }
        (13, 2) => {
            let parsed = aoc2021::d13::parse(input);
            let output = aoc2021::d13::part2(parsed);
            println!("{output}");
        }
        (14, 1) => {
            let parsed = aoc2021::d14::parse(input);
            let output = aoc2021::d14::part1(parsed);
            println!("{output}");
        }
        (14, 2) => {
            let parsed = aoc2021::d14::parse(input);
            let output = aoc2021::d14::part2(parsed);
            println!("{output}");
        }
        (15, 1) => {
            let parsed = aoc2021::d15::parse(input);
            let output = aoc2021::d15::part1(parsed);
            println!("{output}");
        }
        (15, 2) => {
            let parsed = aoc2021::d15::parse(input);
            let output = aoc2021::d15::part2(parsed);
            println!("{output}");
        }
        (16, 1) => {
            let parsed = aoc2021::d16::parse(input);
            let output = aoc2021::d16::part1(parsed);
            println!("{output}");
        }
        (16, 2) => {
            let parsed = aoc2021::d16::parse(input);
            let output = aoc2021::d16::part2(parsed);
            println!("{output}");
        }
        (17, 1) => {
            let parsed = aoc2021::d17::parse(input);
            let output = aoc2021::d17::part1(parsed);
            println!("{output}");
        }
        (17, 2) => {
            let parsed = aoc2021::d17::parse(input);
            let output = aoc2021::d17::part2(parsed);
            println!("{output}");
        }
        (18, 1) => {
            let parsed = aoc2021::d18::parse(input);
            let output = aoc2021::d18::part1(parsed);
            println!("{output}");
        }
        (18, 2) => {
            let parsed = aoc2021::d18::parse(input);
            let output = aoc2021::d18::part2(parsed);
            println!("{output}");
        }
        (19, 1) => {
            let parsed = aoc2021::d19::parse(input);
            let output = aoc2021::d19::part1(parsed);
            println!("{output}");
        }
        (19, 2) => {
            let parsed = aoc2021::d19::parse(input);
            let output = aoc2021::d19::part2(parsed);
            println!("{output}");
        }
        (20, 1) => {
            let parsed = aoc2021::d20::parse(input);
            let output = aoc2021::d20::part1(parsed);
            println!("{output}");
        }
        (20, 2) => {
            let parsed = aoc2021::d20::parse(input);
            let output = aoc2021::d20::part2(parsed);
            println!("{output}");
        }
        (21, 1) => {
            let parsed = aoc2021::d21::parse(input);
            let output = aoc2021::d21::part1(parsed);
            println!("{output}");
        }
        (21, 2) => {
            let parsed = aoc2021::d21::parse(input);
            let output = aoc2021::d21::part2(parsed);
            println!("{output}");
        }
        (22, 1) => {
            let parsed = aoc2021::d22::parse(input);
            let output = aoc2021::d22::part1(parsed);
            println!("{output}");
        }
        (22, 2) => {
            let parsed = aoc2021::d22::parse(input);
            let output = aoc2021::d22::part2(parsed);
            println!("{output}");
        }
        (23, 1) => {
            let parsed = aoc2021::d23::parse(input);
            let output = aoc2021::d23::part1(parsed);
            println!("{output}");
        }
        (23, 2) => {
            let parsed = aoc2021::d23::parse(input);
            let output = aoc2021::d23::part2(parsed);
            println!("{output}");
        }
        (24, 1) => {
            let parsed = aoc2021::d24::parse(input);
            let output = aoc2021::d24::part1(parsed);
            println!("{output}");
        }
        (24, 2) => {
            let parsed = aoc2021::d24::parse(input);
            let output = aoc2021::d24::part2(parsed);
            println!("{output}");
        }
        (25, 1) => {
            let parsed = aoc2021::d25::parse(input);
            let output = aoc2021::d25::part1(parsed);
            println!("{output}");
        }
        (25, 2) => {
            let parsed = aoc2021::d25::parse(input);
            let output = aoc2021::d25::part2(parsed);
            println!("{output}");
        }
        _ => unimplemented!(),
    }
}

/// The CLI arguments allowed.
#[derive(FromArgs, Debug)]
struct Args {
    /// specifies the day (255 runs all parts)
    #[argh(option, short = 'd')]
    day: u8,

    /// specifies the part
    #[argh(option, short = 'p', default = "1")]
    part: u8,

    /// use the day's example input from examples/
    #[argh(switch, short = 'e')]
    example: bool,

    /// specify an alternate input file
    #[argh(option, short = 'i')]
    input: Option<String>,
}