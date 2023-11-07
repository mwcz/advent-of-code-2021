//! A solution to day 25 year 2021.
//! https://adventofcode.com/2021/day/25

#[cfg(feature = "visualize")]
use console_engine::{ConsoleEngine, KeyCode};
use std::convert::From;
use std::fmt::Display;
use std::io::{self, Lines, StdinLock};
use std::time::Duration;

type Model = SeaFloor;
type Answer = u32;

pub fn parse(input: String) -> Model {
    SeaFloor::new(input)
}

pub fn part1(mut floor: Model) -> Answer {
    while !floor.locked {
        floor.step();
    }

    floor.steps
}

pub fn part2(input: Model) -> String {
    "merry christmas!".to_string()
}

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

pub struct SeaFloor {
    map: Vec<Vec<Cucumber>>,
    moves: Vec<Vec<bool>>,
    pub steps: u32,
    locked: bool,
    #[cfg(feature = "visualize")]
    engine: ConsoleEngine,
}

impl SeaFloor {
    fn new(input_text: String) -> Self {
        let map: Vec<Vec<Cucumber>> = input_text
            .lines()
            .map(|line| line.chars().map(Cucumber::from).collect())
            .collect();

        let width = map[0].len();
        let height = map.len();

        #[cfg(feature = "visualize")]
        let fps: u32 = 14;

        #[cfg(feature = "visualize")]
        let mut engine = ConsoleEngine::init(width as u32, height as u32, fps).unwrap();

        let moves = vec![vec![false; width]; height];

        Self {
            map,
            moves,
            steps: 0,
            locked: false,
            #[cfg(feature = "visualize")]
            engine,
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
                if let &Cucumber::East = c {
                    if let Cucumber::Empty = self.map[y][(x + 1) % self.map[y].len()] {
                        self.moves[y][x] = true;
                        moves_exist = true;
                    }
                }
            }
        }

        moves_exist
    }

    fn detect_south_moves(&mut self) -> bool {
        let mut moves_exist = false;
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if let &Cucumber::South = c {
                    if let Cucumber::Empty = self.map[(y + 1) % self.map.len()][x] {
                        self.moves[y][x] = true;
                        moves_exist = true;
                    }
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
        // println!("{}", self);
        self.apply_east_moves();

        self.reset_moves();
        let had_south_moves = self.detect_south_moves();
        // println!("{}", self);
        self.apply_south_moves();

        // #[cfg(feature = "visualize")]
        // println!("{}", self);
        // std::thread::sleep(Duration::from_millis(250));

        #[cfg(feature = "visualize")]
        {
            self.engine.clear_screen();
            self.engine.print(0, 0, self.to_string().as_str());
            self.engine.draw();
            self.engine.wait_frame();
        }

        self.locked = !(had_east_moves || had_south_moves);
    }
}

impl Display for SeaFloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Map")?;

        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if self.moves[y][x] {
                    write!(f, "\x1b[0;91m{}\x1b[0m", c)?;
                } else {
                    write!(f, "{}", c)?;
                }
            }
            writeln!(f)?;
        }

        write!(f, "Step: {}", &self.steps)?;

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
