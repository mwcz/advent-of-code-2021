use std::{
    fmt::Display,
    io::Write,
    ops::{Deref, DerefMut},
    str::Split,
};

#[derive(Debug)]
struct Grid(Vec<Point>);

impl Deref for Grid {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pointmap: std::collections::HashSet<&Point> = std::collections::HashSet::new();
        let mut x_max = 0;
        let mut y_max = 0;
        for point in self.iter() {
            pointmap.insert(point);
            x_max = x_max.max(point.x);
            y_max = y_max.max(point.y);
        }
        let width = x_max + 1;
        let height = y_max + 1;

        let mut msg = vec![vec![' '; width]; height];

        for p in pointmap {
            msg[p.y][p.x] = '▮';
        }

        write!(
            f,
            "{}",
            msg.iter()
                .map(|row| row.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )?;

        Ok(())
    }
}

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

    let mut grid: Grid = Grid(
        input_parts
            .next()
            .unwrap()
            .lines()
            .map(|xy| xy.split(',').into())
            .collect::<Vec<Point>>(),
    );

    let moves: Vec<Move> = input_parts.next().unwrap().lines().map(Move::new).collect();

    for mov in &moves {
        for point in grid.iter_mut() {
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
    }

    println!("{grid}");
}
