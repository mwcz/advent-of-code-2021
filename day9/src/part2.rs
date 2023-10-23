use std::{fmt::Display, io};

const WALL: u8 = 10;

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

fn main() {
    let lines = io::stdin().lines();

    let mut low_points: Vec<Coord> = Vec::new();

    let terrain = lines
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    for y in 0..terrain.len() {
        let row = &terrain[y];
        let row_up = terrain.get(y.wrapping_sub(1));
        let row_down = terrain.get(y.wrapping_add(1));

        for x in 0..terrain.get(0).unwrap().len() {
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
        let mut basin_map = vec![vec![0 as u8; terrain[0].len()]; terrain.len()];

        // mark the low_point on the map
        basin_map[low_point.y as usize][low_point.x as usize] = 1;

        // println!("basin_members);
        // for mem in basin_members {
        //     println!("  {:?}", mem);
        // }

        // mark the initial low point as the only member of the basin
        // basin_members[low_point.y as usize][low_point.x as usize] = 1;

        let mut basin_members = vec![low_point.clone(); 1];

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

        println!("basin map:");
        for region in basin_map {
            for p in region {
                print!("{}", if p > 0 { "█" } else { "░" });
            }
            print!("\n");
        }

        basins.push(basin_members);
    });

    basins.sort_by(|basin_a, basin_b| basin_b.len().cmp(&basin_a.len()));

    let sum: usize = basins[0..3]
        .iter()
        .map(|b| b.len())
        .reduce(|acc, item| acc * item)
        .unwrap();

    println!("Answer: {}", sum);
}

/// Given an x and y point, find any adjacent (up, down, left, right) values that are members of
/// the same basin.
fn adjacents_in_basin(
    c: Coord,
    terrain: &Vec<Vec<u8>>,
    basin_map: &mut Vec<Vec<u8>>,
) -> Vec<Coord> {
    // create a vec of coords pointing in every cardinal direction, then filter out the ones that
    // aren't part of the basin (due to being equal to the breadcrumb, or out of bounds, or equal
    // to 9)
    let to_search: Vec<Coord> = vec![c.inc_x(), c.dec_x(), c.inc_y(), c.dec_y()]
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
