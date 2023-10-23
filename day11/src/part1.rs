use std::{
    cell::RefCell,
    fmt::Display,
    io::{self, Lines, StdinLock},
};

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

#[derive(Debug)]
struct Octopi {
    levels: Vec<Vec<u8>>,
}

impl Octopi {
    /// Run a step and return the number of flashes.
    fn step(&mut self) -> usize {
        // where flashes are happening during this step
        let mut flashes: Vec<(usize, usize)> = Vec::new();
        let mut flashes_pending: Vec<(usize, usize)> = Vec::new();
        let mut flashes_resolved: Vec<(usize, usize)> = Vec::new();

        for (y, row) in self.levels.iter_mut().enumerate() {
            for (x, energy) in row.iter_mut().enumerate() {
                *energy += 1;
                if energy == &10 {
                    flashes.push((x, y));
                }
            }
        }

        while !flashes.is_empty() {
            for (x, y) in &flashes {
                for (xd, yd) in [
                    (-1, -1), // tl
                    (0, -1),  // t
                    (1, -1),  // tr
                    (-1, 0),  // l
                    (1, 0),   // r
                    (-1, 1),  // bl
                    (0, 1),   // b
                    (1, 1),   // br
                ] {
                    if let (Some(new_x), Some(new_y)) =
                        (x.checked_add_signed(xd), y.checked_add_signed(yd))
                    {
                        if let Some(row) = self.levels.get_mut(new_y) {
                            if let Some(level) = row.get_mut(new_x) {
                                *level += 1;
                                if level == &10 {
                                    flashes_pending.push((new_x, new_y));
                                }
                            }
                        }
                    }
                }
            }
            // move all elements of flashes into flashes_resolved
            flashes_resolved.append(&mut flashes);
            flashes.append(&mut flashes_pending);
        }

        for (x, y) in &flashes_resolved {
            self.levels[*y][*x] = 0;
        }

        flashes_resolved.len()
    }
}

impl Display for Octopi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.levels {
            for energy in row {
                write!(f, "{}", energy)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl From<Lines<StdinLock<'_>>> for Octopi {
    fn from(lines: Lines<StdinLock>) -> Self {
        let levels: Vec<Vec<u8>> = lines
            .map(|line| {
                line.unwrap()
                    .trim()
                    .split("")
                    .filter(|s| s.len() > 0)
                    .map(|c| c.parse::<u8>().unwrap())
                    .collect()
            })
            .collect();

        Octopi { levels }
    }
}

fn main() {
    let lines = io::stdin().lines();

    let mut octo = Octopi::from(lines);

    let mut sum = 0;

    for _ in 0..100 {
        sum += octo.step();
        println!("{}", octo);
    }

    println!("total: {sum}");
}
