#![feature(stdin_forwarders)]
use std::io;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Line {
    a: Point,
    b: Point,
}

fn main() {
    ///////////////////
    //  parse input  //
    ///////////////////

    let input_lines = io::stdin().lines();
    let mut max_x = 0;
    let mut max_y = 0;
    let lines: Vec<Line> = input_lines
        .map(|input_line| {
            let input_str = input_line.unwrap();
            let points: Vec<Point> = input_str
                .split(" -> ")
                .map(|xy| {
                    let xy_split: Vec<i32> =
                        xy.split(",").map(|m| m.parse::<i32>().unwrap()).collect();
                    let x = xy_split.first().unwrap().clone();
                    let y = xy_split.last().unwrap().clone();
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

    //////////////////
    //  create map  //
    //////////////////

    let mut map: Vec<Vec<i32>> = vec![vec![0; (max_x + 1) as usize]; (max_y + 1) as usize];
    let mut danger_count = 0;

    for line in &lines {
        println!("drawing line {:?}", line);
        if line.a.x == line.b.x {
            // vertical
            let x = line.a.x;
            for y in line.a.y.min(line.b.y)..line.a.y.max(line.b.y) + 1 {
                println!("  drawing point {},{}", x, y);
                map[y as usize][x as usize] += 1;
                if map[y as usize][x as usize] == 2 {
                    danger_count += 1;
                }
            }
        } else if line.a.y == line.b.y {
            // horizontal
            let y = line.a.y;
            for x in line.a.x.min(line.b.x)..line.a.x.max(line.b.x) + 1 {
                println!("  drawing point {},{}", x, y);
                map[y as usize][x as usize] += 1;
                if map[y as usize][x as usize] == 2 {
                    danger_count += 1;
                }
            }
        } else {
            // diagonals
            let mut y = line.a.y;
            let mut x = line.a.x;

            let x_parity = if line.a.x < line.b.x { 1 } else { -1 };
            let y_parity = if line.a.y < line.b.y { 1 } else { -1 };

            loop {
                println!("  drawing point {},{}", x, y);
                map[y as usize][x as usize] += 1;
                if map[y as usize][x as usize] == 2 {
                    danger_count += 1;
                }
                if y == line.b.y || x == line.b.x {
                    break;
                }
                x += x_parity;
                y += y_parity;
            }
        }
    }

    ////////////////////
    //  print output  //
    ////////////////////

    println!("Lines:");
    for line in &lines {
        println!("  {:?}", line);
    }

    println!("Map:");
    for row in &map {
        for c in row {
            if c == &0 {
                print!(".");
            } else {
                print!("{}", c);
            }
        }
        println!("");
    }

    println!("Dimensions: {}x{}", max_x, max_y);

    println!("Danger count: {}", danger_count);
}
