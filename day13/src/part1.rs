use std::str::Split;

fn main() {
    // read file specified in args[1] or if none is specified, read from stdin
    let lines: String = std::env::args()
        .nth(1)
        .map(|f| std::fs::read_to_string(f).unwrap())
        .or_else(|| {
            Some(
                std::io::stdin()
                    .lines()
                    .map(|line| line.unwrap())
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
        })
        .unwrap();

    let mut input_parts = lines.split("\n\n");

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Point {
        x: usize,
        y: usize,
    }

    impl From<Split<'_, char>> for Point {
        fn from(s: Split<char>) -> Self {
            let mut coords = s.clone();
            let x = coords.next().unwrap().parse::<usize>().unwrap();
            let y = coords.next().unwrap().parse::<usize>().unwrap();
            Self { x, y }
        }
    }

    #[derive(Debug)]
    enum Axis {
        X,
        Y,
    }

    impl From<char> for Axis {
        fn from(value: char) -> Self {
            if value == 'x' {
                Self::X
            } else {
                Self::Y
            }
        }
    }

    #[derive(Debug)]
    struct Move {
        axis: Axis,
        val: usize,
    }

    impl Move {
        fn new(line: &str) -> Self {
            let mut parts = line.split('=');
            let axis = parts.next().unwrap().chars().last().unwrap().into();
            let val = parts.next().unwrap().parse().unwrap();
            Self { axis, val }
        }
    }

    let mut points: Vec<Point> = input_parts
        .next()
        .unwrap()
        .lines()
        .map(|xy| xy.split(',').into())
        .collect();

    let moves: Vec<Move> = input_parts.next().unwrap().lines().map(Move::new).collect();

    dbg!(&points);
    dbg!(&moves);

    for mov in &moves {
        for point in points.iter_mut() {
            match mov.axis {
                Axis::X => {
                    if point.x > mov.val {
                        point.x -= 2 * (point.x - mov.val)
                    }
                }
                Axis::Y => {
                    if point.y > mov.val {
                        point.y -= 2 * (point.y - mov.val)
                    }
                }
            }
        }
        if true {
            break;
        }
    }

    // count the number of occurrences of each point
    let mut counts: std::collections::HashMap<&Point, usize> = std::collections::HashMap::new();
    for point in &points {
        *counts.entry(point).or_insert(0) += 1;
    }
    dbg!(&points
        .iter()
        .map(|p| format!("{},{}", p.x, p.y))
        .collect::<Vec<String>>());
    dbg!(counts.len());
}
