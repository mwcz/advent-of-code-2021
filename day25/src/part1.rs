use std::convert::From;
use std::fmt::Display;
use std::io::{self, Lines, StdinLock};

#[derive(Debug, Copy, Clone)]
enum Cucumber {
    East,
    South,
    Empty,
}

impl Display for Cucumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cucumber::East => '>',
                Cucumber::South => 'v',
                Cucumber::Empty => '.',
            }
        )
    }
}

struct SeaFloor {
    map: Vec<Vec<Cucumber>>,
    moves: Vec<Vec<bool>>,
    steps: u32,
    locked: bool,
}

impl SeaFloor {
    fn new(input_text: Lines<StdinLock>) -> Self {
        let map: Vec<Vec<Cucumber>> = input_text
            .map(|line| line.unwrap().chars().map(|c| Cucumber::from(c)).collect())
            .collect();
        let moves = vec![vec![false; map[0].len()]; map.len()];
        Self {
            map,
            moves,
            steps: 0,
            locked: false,
        }
    }

    fn reset_moves(&mut self) {
        for row in &mut self.moves {
            row.fill(false);
        }
    }

    fn detect_east_moves(&mut self) -> bool {
        let mut moves_exist = false;
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                // which direction is this cucumber headed?
                match c {
                    &Cucumber::East => match self.map[y][(x + 1) % self.map[y].len()] {
                        Cucumber::Empty => {
                            self.moves[y][x] = true;
                            moves_exist = true;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        moves_exist
    }

    fn detect_south_moves(&mut self) -> bool {
        let mut moves_exist = false;
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                match c {
                    // if this cucumber is heading south, and the southern space is Empty, go for
                    // it
                    &Cucumber::South => match self.map[(y + 1) % self.map.len()][x] {
                        Cucumber::Empty => {
                            self.moves[y][x] = true;
                            moves_exist = true;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        moves_exist
    }

    fn apply_east_moves(&mut self) {
        for (y, row) in self.moves.iter_mut().enumerate() {
            for (x, should_move) in row.iter_mut().enumerate() {
                if *should_move {
                    let len = self.map[y].len();
                    let c = self.map[y][x];
                    self.map[y][(x + 1) % len] = c;
                    self.map[y][x] = Cucumber::Empty;
                }
            }
        }
    }

    fn apply_south_moves(&mut self) {
        for (y, row) in self.moves.iter_mut().enumerate() {
            for (x, should_move) in row.iter_mut().enumerate() {
                if *should_move {
                    let len = self.map.len();
                    let c = self.map[y][x];
                    self.map[(y + 1) % len][x] = c;
                    self.map[y][x] = Cucumber::Empty;
                }
            }
        }
    }

    fn step(&mut self) {
        self.steps += 1;

        self.reset_moves();
        let had_east_moves = self.detect_east_moves();
        println!("{}", self);
        self.apply_east_moves();

        self.reset_moves();
        let had_south_moves = self.detect_south_moves();
        println!("{}", self);
        self.apply_south_moves();

        self.locked = !(had_east_moves || had_south_moves);
    }
}

impl Display for SeaFloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Map\n").unwrap();

        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if self.moves[y][x] {
                    write!(f, "\x1b[0;91m{}\x1b[0m", c).unwrap();
                } else {
                    write!(f, "{}", c).unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }

        write!(f, "Step: {}", &self.steps).unwrap();

        Ok(())
    }
}

impl From<char> for Cucumber {
    fn from(c: char) -> Self {
        match c {
            '>' => Cucumber::East,
            'v' => Cucumber::South,
            _ => Cucumber::Empty,
        }
    }
}

fn main() {
    let lines = io::stdin().lines();

    let mut t = SeaFloor::new(lines);

    while !t.locked {
        t.step();
    }
}
