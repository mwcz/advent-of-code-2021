//! A solution to day 5 year 2021.
//! https://adventofcode.com/2021/day/5

type Model = (Vec<Line>, Point);
type Answer = i32;

pub fn parse(input: String) -> Model {
    let input_lines = input.lines();
    let mut max_x = 0;
    let mut max_y = 0;
    let lines: Vec<Line> = input_lines
        .map(|input_line| {
            let points: Vec<Point> = input_line
                .split(" -> ")
                .map(|xy| {
                    let xy_split: Vec<usize> =
                        xy.split(',').map(|m| m.parse::<usize>().unwrap()).collect();
                    let x = *xy_split.first().unwrap();
                    let y = *xy_split.last().unwrap();
                    max_x = if x > max_x { x } else { max_x };
                    max_y = if y > max_y { y } else { max_y };
                    Point { x, y }
                })
                .collect();

            Line {
                a: (*points.first().unwrap()).clone(),
                b: (*points.last().unwrap()).clone(),
            }
        })
        .collect();

    (lines, Point { x: max_x, y: max_y })
}

pub fn part1((lines, max): Model) -> Answer {
    let mut map: Vec<Vec<usize>> = vec![vec![0; max.x + 1]; max.y + 1];
    let mut danger_count = 0;

    for line in &lines {
        // println!("drawing line {:?}", line);
        if line.a.x == line.b.x {
            let x = line.a.x;
            (line.a.y.min(line.b.y)..line.a.y.max(line.b.y) + 1).for_each(|y| {
                // println!("  drawing point {},{}", x, y);
                map[y][x] += 1;
                if map[y][x] == 2 {
                    danger_count += 1;
                }
            });
        } else if line.a.y == line.b.y {
            let y = line.a.y;
            for x in line.a.x.min(line.b.x)..line.a.x.max(line.b.x) + 1 {
                // println!("  drawing point {},{}", x, y);
                map[y][x] += 1;
                if map[y][x] == 2 {
                    danger_count += 1;
                }
            }
        } // skip diagonal lines (for now)
    }

    danger_count
}

pub fn part2((lines, max): Model) -> Answer {
    let mut map: Vec<Vec<i32>> = vec![vec![0; (max.x + 1)]; (max.y + 1)];
    let mut danger_count = 0;

    for line in &lines {
        // println!("drawing line {:?}", line);
        if line.a.x == line.b.x {
            // vertical
            let x = line.a.x;
            (line.a.y.min(line.b.y)..line.a.y.max(line.b.y) + 1).for_each(|y| {
                // println!("  drawing point {},{}", x, y);
                map[y][x] += 1;
                if map[y][x] == 2 {
                    danger_count += 1;
                }
            });
        } else if line.a.y == line.b.y {
            // horizontal
            let y = line.a.y;
            for x in line.a.x.min(line.b.x)..line.a.x.max(line.b.x) + 1 {
                // println!("  drawing point {},{}", x, y);
                map[y][x] += 1;
                if map[y][x] == 2 {
                    danger_count += 1;
                }
            }
        } else {
            // diagonals
            let mut y = line.a.y;
            let mut x = line.a.x;

            let x_parity: i8 = if line.a.x < line.b.x { 1 } else { -1 };
            let y_parity: i8 = if line.a.y < line.b.y { 1 } else { -1 };

            loop {
                // println!("  drawing point {},{}", x, y);
                map[y][x] += 1;
                if map[y][x] == 2 {
                    danger_count += 1;
                }
                if y == line.b.y || x == line.b.x {
                    break;
                }
                match x_parity {
                    1 => x += 1,
                    -1 => x = x.checked_sub(1).unwrap_or(x),
                    _ => {}
                };
                match y_parity {
                    1 => y += 1,
                    -1 => y = y.checked_sub(1).unwrap_or(y),
                    _ => {}
                };
            }
        }
    }

    danger_count
}

#[derive(Debug, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct Line {
    a: Point,
    b: Point,
}
