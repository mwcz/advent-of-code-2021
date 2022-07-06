use std::io;

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
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
                    let xy_split: Vec<usize> =
                        xy.split(",").map(|m| m.parse::<usize>().unwrap()).collect();
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

    let mut map: Vec<Vec<usize>> = vec![vec![0; max_x + 1]; max_y + 1];
    let mut danger_count = 0;

    for line in &lines {
        println!("drawing line {:?}", line);
        if line.a.x == line.b.x {
            let x = line.a.x;
            for y in line.a.y.min(line.b.y)..line.a.y.max(line.b.y) + 1 {
                println!("  drawing point {},{}", x, y);
                map[y][x] += 1;
                if map[y][x] == 2 {
                    danger_count += 1;
                }
            }
        } else if line.a.y == line.b.y {
            let y = line.a.y;
            for x in line.a.x.min(line.b.x)..line.a.x.max(line.b.x) + 1 {
                println!("  drawing point {},{}", x, y);
                map[y][x] += 1;
                if map[y][x] == 2 {
                    danger_count += 1;
                }
            }
        } // skip diagonal lines (for now)
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
        println!("  {:?}", row);
    }

    println!("Dimensions: {}x{}", max_x, max_y);

    println!("Danger count: {}", danger_count);
}
