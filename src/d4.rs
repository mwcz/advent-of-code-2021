//! A solution to day 4 year 2021.
//! https://adventofcode.com/2021/day/4

type Model = (Vec<u32>, Vec<Board>);
type Answer = u32;

pub fn parse(input: String) -> Model {
    let mut lines = input.lines();

    // parse the numbers on the first line
    let numbers: Vec<u32> = lines
        .next() // get the first line
        .unwrap() // unwrap Option
        .split(',')
        .map(|n| n.parse::<u32>().unwrap()) // parse to u32
        .collect();

    // skip the guaranteed empty line
    lines.next();

    // parse the boards text into a workable data structure
    let boards: Vec<Board> = lines
        .collect::<Vec<&str>>() // collect into a Vec of Strings, couldn't figure out how to split a std::io::Lines
        .split(|line| line.is_empty()) // split on empty lines
        .map(|board| {
            let board: [[u32; BOARD_SIZE]; BOARD_SIZE] = board
                .iter()
                .map(|row| {
                    let row: [u32; BOARD_SIZE] = row
                        .trim()
                        .to_string()
                        .split_whitespace()
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap();
                    row
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            board
        })
        .map(Board::create)
        .collect();

    (numbers, boards)
}

pub fn part1((numbers, mut boards): Model) -> Answer {
    let mut first_matched_board: (Option<u32>, Option<Board>) = (None, None);

    'nums: for n in numbers {
        for b in &mut boards {
            // report n to board
            b.mark(n);
            // check if board has bingo
            if b.has_bingo() {
                first_matched_board = (Some(n), Some(b.clone()));
                break 'nums;
            }
            // if it has bingo, get the unmarked numbers, sum them, and multiply by n
        }
    }

    let final_number = &first_matched_board.0.unwrap();
    let unmarked_sum = first_matched_board
        .1
        .unwrap()
        .unmarked()
        .iter()
        .sum::<u32>();
    let answer = unmarked_sum * final_number;

    // println!("unmarked sum: {}", unmarked_sum);
    // println!("final number: {}", final_number);
    // println!("answer: {}", answer);

    answer
}

pub fn part2((numbers, mut boards): Model) -> Answer {
    let mut last_matched_board: (Option<u32>, Option<Board>) = (None, None);

    for n in numbers {
        for b in &mut boards {
            if !b.has_bingo() {
                b.mark(n);
                if b.has_bingo() {
                    last_matched_board = (Some(n), Some(b.clone()));
                }
            }
        }
    }

    let final_number = &last_matched_board.0.unwrap();
    let unmarked_sum = last_matched_board.1.unwrap().unmarked().iter().sum::<u32>();
    let answer = unmarked_sum * final_number;

    // println!("unmarked sum: {}", unmarked_sum);
    // println!("final number: {}", final_number);
    // println!("answer: {}", answer);

    answer
}

const BOARD_SIZE: usize = 5;

#[derive(Debug, Clone)]
pub struct Board {
    squares: [[u32; BOARD_SIZE]; BOARD_SIZE],
    score: [[u8; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    /// Check if this board has bingo.
    fn has_bingo(&self) -> bool {
        for y in 0..self.score.len() {
            let mut row_match = true;
            for x in 0..self.score[y].len() {
                row_match = row_match && self.score[y][x] == 1;
            }
            if row_match {
                return true;
            }
        }

        for x in 0..self.score[0].len() {
            let mut col_match = true;
            for y in 0..self.score.len() {
                col_match = col_match && (self.score[y][x] == 1);
            }
            if col_match {
                return true;
            }
        }

        false
    }

    /// If the given number exists on this board, mark it.
    fn mark(&mut self, num: u32) {
        let mut match_x: usize = 0;
        let mut match_y: usize = 0;

        let row_match = self.squares.iter().enumerate().find(|(y, row)| {
            let num_match = row.iter().enumerate().find(|(_, &n)| n == num);

            match num_match {
                Some(n) => {
                    match_x = n.0;
                    match_y = *y;
                    n.1 == &num
                }
                None => false,
            }
        });

        if row_match.is_some() {
            self.score[match_y][match_x] = 1;
        }
    }

    /// Get a collection of all the unmarked numbers on this board.
    fn unmarked(&self) -> Vec<u32> {
        let mut un: Vec<u32> = Vec::new();
        for y in 0..self.squares.len() {
            for x in 0..self.squares[y].len() {
                if self.score[y][x] == 0 {
                    un.push(self.squares[y][x]);
                }
            }
        }
        un
    }

    fn create(numbers: [[u32; BOARD_SIZE]; BOARD_SIZE]) -> Self {
        Self {
            squares: numbers,
            score: [[0; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}
