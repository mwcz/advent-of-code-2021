//! A solution to day 9 year 2021.
//! https://adventofcode.com/2021/day/9

#[cfg(feature = "visualize")]
use console_engine::{ConsoleEngine, KeyCode};
use std::fmt::Display;

type Model = Vec<Vec<u8>>;
type Answer = usize;

const WALL: u8 = 10;

pub fn parse(input: String) -> Model {
    let rows = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    rows
}

pub fn part1(rows: Model) -> Answer {
    let mut risks: Vec<u8> = Vec::new();

    for y in 0..rows.len() {
        let row = &rows[y];
        let row_up = rows.get(y.wrapping_sub(1));
        let row_down = rows.get(y.wrapping_add(1));

        for x in 0..rows.first().unwrap().len() {
            let h_left = row.get(x.wrapping_sub(1)).unwrap_or(&WALL);
            let h_right = row.get(x.wrapping_add(1)).unwrap_or(&WALL);

            let h = row[x];

            let h_up = match row_up {
                Some(row) => row.get(x).unwrap(),
                None => &WALL,
            };

            let h_down = match row_down {
                Some(row) => row.get(x).unwrap(),
                None => &WALL,
            };

            if h < *h_up && h < *h_down && h < *h_left && h < *h_right {
                // low point found
                risks.push(h + 1);
            }
        }
    }

    risks.iter().map(|r| *r as u32).sum::<u32>() as usize
}

pub fn part2(terrain: Model) -> Answer {
    let mut low_points: Vec<Coord> = Vec::new();
    let width = terrain[0].len();
    let height = terrain.len();

    #[cfg(feature = "visualize")]
    let fps: u32 = 14;

    #[cfg(feature = "visualize")]
    let mut engine = ConsoleEngine::init(width as u32, height as u32, fps).unwrap();

    for y in 0..terrain.len() {
        let row = &terrain[y];
        let row_up = terrain.get(y.wrapping_sub(1));
        let row_down = terrain.get(y.wrapping_add(1));

        for x in 0..terrain.first().unwrap().len() {
            let h_left = row.get(x.wrapping_sub(1)).unwrap_or(&WALL);
            let h_right = row.get(x.wrapping_add(1)).unwrap_or(&WALL);

            let h = row[x];

            let h_up = match row_up {
                Some(row) => row.get(x).unwrap(),
                None => &WALL,
            };

            let h_down = match row_down {
                Some(row) => row.get(x).unwrap(),
                None => &WALL,
            };

            if h < *h_up && h < *h_down && h < *h_left && h < *h_right {
                // low point found
                low_points.push(Coord {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
            }
        }
    }

    let mut basins: Vec<Vec<Coord>> = Vec::new();

    low_points.iter().for_each(|low_point| {
        // println!("  {:?}", low_point);

        let mut i: usize = 0;

        // create a 2D vec of the same dimensions as the terrain
        let mut basin_map = vec![vec![0; terrain[0].len()]; terrain.len()];

        // mark the low_point on the map
        basin_map[low_point.y as usize][low_point.x as usize] = 1;

        // println!("basin_members);
        // for mem in basin_members {
        //     println!("  {:?}", mem);
        // }

        // mark the initial low point as the only member of the basin
        // basin_members[low_point.y as usize][low_point.x as usize] = 1;

        let mut basin_members = vec![*low_point; 1];

        loop {
            let loc = basin_members[i];
            i += 1;

            // println!("    {:?}", loc);

            let mut new_basin_members = adjacents_in_basin(loc, &terrain, &mut basin_map);

            basin_members.append(&mut new_basin_members);

            if i >= basin_members.len() {
                break;
            }
        }

        // println!("basin members:");
        // for member in &basin_members {
        //     println!("{}", member);
        // }

        #[cfg(feature = "visualize")]
        {
            let mut basin_map_str = String::new();
            for region in basin_map {
                for p in region {
                    basin_map_str.push(if p > 0 { '█' } else { '░' });
                }
                basin_map_str.push('\n');
            }

            engine.clear_screen();
            engine.print(0, 0, &basin_map_str);
            engine.draw();
            engine.wait_frame();
        }

        basins.push(basin_members);
    });

    basins.sort_by_key(|basin_b| std::cmp::Reverse(basin_b.len()));

    let sum: usize = basins[0..3]
        .iter()
        .map(|b| b.len())
        .reduce(|acc, item| acc * item)
        .unwrap();

    sum
}

/// A location struct with attached breadcrumb (the point that preceeded this point in the search).
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coord {
    fn inc_x(&self) -> Self {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn inc_y(&self) -> Self {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn dec_x(&self) -> Self {
        Coord {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn dec_y(&self) -> Self {
        Coord {
            x: self.x,
            y: self.y - 1,
        }
    }
}

/// Given an x and y point, find any adjacent (up, down, left, right) values that are members of
/// the same basin.
fn adjacents_in_basin(c: Coord, terrain: &Vec<Vec<u8>>, basin_map: &mut [Vec<u8>]) -> Vec<Coord> {
    // create a vec of coords pointing in every cardinal direction, then filter out the ones that
    // aren't part of the basin (due to being equal to the breadcrumb, or out of bounds, or equal
    // to 9)
    let to_search: Vec<Coord> = [c.inc_x(), c.dec_x(), c.inc_y(), c.dec_y()]
        .iter()
        .filter(|c| in_range(c, terrain))
        .filter(|c| basin_map[c.y as usize][c.x as usize] == 0) // filter out already visited
        .filter(|c| terrain[c.y as usize][c.x as usize] < 9) // not a wall
        .cloned()
        .collect();

    // mark this position in the basin map
    to_search
        .iter()
        .for_each(|c| basin_map[c.y as usize][c.x as usize] = 1);

    to_search
}

fn in_range(loc: &Coord, terrain: &Vec<Vec<u8>>) -> bool {
    loc.x >= 0
        && loc.y >= 0
        && loc.x < terrain[0].len().try_into().unwrap()
        && loc.y < terrain.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d9");

    #[test]
    fn d9p1_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 633);
    }

    #[test]
    fn d9p2_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 1050192);
    }
}
