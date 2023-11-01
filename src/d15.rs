//! A solution to day 15 year 2021.
//! https://adventofcode.com/2021/day/15

use crate::answer::Answer;
use pathfinding::prelude::dijkstra;

type Parsed = Vec<Vec<u32>>;

pub fn parse(input: String) -> Parsed {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    grid
}

pub fn part1(grid: Parsed) -> impl Answer {
    let start: (usize, usize) = (0, 0);
    let end: (usize, usize) = (grid.len() - 1, grid[0].len() - 1);

    let path = dijkstra(
        &start,
        |&(x, y)| {
            let mut successors = vec![];
            // go up if y > 0
            if y > 0 {
                successors.push(((x, y - 1), grid[y - 1][x]));
            }
            // go left if x > 0
            if x > 0 {
                successors.push(((x - 1, y), grid[y][x - 1]));
            }
            // go right if x < max_x
            if x < end.0 {
                successors.push(((x + 1, y), grid[y][x + 1]));
            }
            // go down if y < max_y
            if y < end.1 {
                successors.push(((x, y + 1), grid[y + 1][x]));
            }

            successors.into_iter()
        },
        |&p| p == end,
    );

    let lowest_risk = path.unwrap().1;
    // println!("{lowest_risk}");

    lowest_risk
}

pub fn part2(grid_part: Parsed) -> impl Answer {
    let orig_width = grid_part.len();
    let orig_height = grid_part[0].len();

    let mut grid: Vec<Vec<u32>> = vec![vec![0; orig_width * 5]; orig_height * 5];

    for rep_x in 0..5u32 {
        for rep_y in 0..5u32 {
            for (y, row) in grid_part.iter().enumerate() {
                for (x, val) in row.iter().enumerate() {
                    let newval = (*val + rep_x + rep_y) % 10 + (*val + rep_x + rep_y) / 10;
                    grid[y + rep_y as usize * orig_height][x + rep_x as usize * orig_width] =
                        newval;
                }
            }
        }
    }

    let start: (usize, usize) = (0, 0);
    let end: (usize, usize) = (grid.len() - 1, grid[0].len() - 1);

    let path = dijkstra(
        &start,
        |&(x, y)| {
            let mut successors = vec![];
            // go up if y > 0
            if y > 0 {
                successors.push(((x, y - 1), grid[y - 1][x]));
            }
            // go left if x > 0
            if x > 0 {
                successors.push(((x - 1, y), grid[y][x - 1]));
            }
            // go right if x < max_x
            if x < end.0 {
                successors.push(((x + 1, y), grid[y][x + 1]));
            }
            // go down if y < max_y
            if y < end.1 {
                successors.push(((x, y + 1), grid[y + 1][x]));
            }

            successors.into_iter()
        },
        |&p| p == end,
    );

    let lowest_risk = path.unwrap().1;

    lowest_risk
}
