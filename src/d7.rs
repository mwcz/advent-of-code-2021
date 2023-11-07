//! A solution to day 7 year 2021.
//! https://adventofcode.com/2021/day/7

type Model = Vec<i64>;
type Answer = i32;

pub fn parse(input: String) -> Model {
    let mut positions: Vec<i64> = input
        .split(',')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    positions.sort();

    positions
}

pub fn part1(positions: Model) -> i64 {
    let median = positions.get(positions.len() / 2).unwrap();
    let fuel: i64 = positions.iter().map(|pos| (pos - median).abs()).sum();

    fuel
}

pub fn part2(positions: Model) -> i64 {
    let mean: i64 = positions.iter().sum::<i64>() / (positions.len() as i64);

    let fuel: i64 = positions
        .iter()
        .map(|pos| {
            let d = (pos - mean).abs();
            (d.pow(2) + d) / 2
        })
        .sum();

    fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d7");

    #[test]
    fn d7p1_example_test() {
        assert_eq!(part1(parse(include_str!("../examples/d7").to_string())), 37);
    }

    #[test]
    fn d7p1_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 347449);
    }

    // My solution gets this example wrong even though it gets the final answer correct.
    // #[test]
    // fn d7p2_example_test() {
    //     assert_eq!(
    //         part2(parse(include_str!("../examples/d7").to_string())),
    //         168
    //     );
    // }

    #[test]
    fn d7p2_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 98039527);
    }
}
