#![feature(stdin_forwarders)]
use std::io;

fn main() {
    let lines = io::stdin().lines();

    let mut stack: Vec<char> = Vec::new();

    let mut scores: Vec<u64> = Vec::new();

    let mut paren_stack = 0;
    let mut brace_stack = 0;
    let mut bracket_stack = 0;
    let mut angle_stack = 0;

    'line: for line in lines {
        stack.clear();
        if let Ok(line) = line {
            for ch in line.chars() {
                match ch {
                    '(' => {
                        paren_stack += 1;
                        stack.push('(');
                    }
                    ')' => {
                        if paren_stack > 0 && stack.last() == Some(&'(') {
                            paren_stack -= 1;
                            stack.pop();
                        } else {
                            continue 'line;
                        }
                    }
                    '{' => {
                        brace_stack += 1;
                        stack.push('{');
                    }
                    '}' => {
                        if brace_stack > 0 && stack.last() == Some(&'{') {
                            brace_stack -= 1;
                            stack.pop();
                        } else {
                            continue 'line;
                        }
                    }
                    '[' => {
                        bracket_stack += 1;
                        stack.push('[');
                    }
                    ']' => {
                        if bracket_stack > 0 && stack.last() == Some(&'[') {
                            bracket_stack -= 1;
                            stack.pop();
                        } else {
                            continue 'line;
                        }
                    }
                    '<' => {
                        angle_stack += 1;
                        stack.push('<');
                    }
                    '>' => {
                        if angle_stack > 0 && stack.last() == Some(&'<') {
                            angle_stack -= 1;
                            stack.pop();
                        } else {
                            continue 'line;
                        }
                    }
                    _ => {}
                }
            }
        }
        let mut score = 0;
        stack.reverse();
        for c in &stack {
            score *= 5;
            score += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            }
        }
        scores.push(score);
    }

    // count unclosed surrounds as errors

    scores.sort();
    let answer = scores[scores.len() / 2];

    println!("{}", answer);
}
