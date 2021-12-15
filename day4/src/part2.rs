#![feature(stdin_forwarders)]
use std::io;

#[derive(Debug, Clone)]
struct Board {
    squares: Vec<Vec<u32>>,
    score: Vec<Vec<u8>>,
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

        match row_match {
            Some(_) => {
                self.score[match_y][match_x] = 1;
            }
            _ => {}
        };

        // for s in &self.score {
        //     println!("{:?}", s);
        // }
        // println!("");
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

    fn create(numbers: &Vec<Vec<u32>>) -> Self {
        Self {
            squares: numbers.clone(),
            score: vec![vec![0; numbers[0].len()]; numbers.len()],
        }
    }
}

fn main() {
    let mut lines = io::stdin().lines();

    // parse the numbers on the first line
    let numbers: Vec<u32> = lines
        .next() // get the first line
        .unwrap() // unwrap Option
        .unwrap() // unwrap Result
        .split(",")
        .map(|n| n.parse::<u32>().unwrap()) // parse to u32
        .collect();

    // skip the guaranteed empty line
    lines.next();

    // parse the boards text into a workable data structure
    let mut boards: Vec<Board> = lines
        .map(|r| r.unwrap()) // unwrap Result
        .collect::<Vec<String>>() // collect into a Vec of Strings, couldn't figure out how to split a std::io::Lines
        .split(|line| line.len() == 0) // split on empty lines
        .map(|board| {
            board
                .iter()
                .map(|row| {
                    row.trim()
                        .to_string()
                        .split_whitespace()
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .map(|numbers| Board::create(&numbers))
        .collect();

    // println!("numbers:");
    // println!("  {:?}\n", numbers);

    // println!("boards ({} found)", boards.len());
    // for board in &boards {
    //     for row in &board.squares {
    //         println!("  {:?}", row);
    //     }
    //     println!("");
    // }

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

    println!("unmarked sum: {}", unmarked_sum);
    println!("final number: {}", final_number);
    println!("answer: {}", answer);
}
