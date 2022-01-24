#![feature(stdin_forwarders)]
use std::io;

fn main() {
    let lines = io::stdin().lines();

    let mut stack: Vec<char> = Vec::new();

    let mut paren_stack = 0;
    let mut paren_error = 0;

    let mut brace_stack = 0;
    let mut brace_error = 0;

    let mut bracket_stack = 0;
    let mut bracket_error = 0;

    let mut angle_stack = 0;
    let mut angle_error = 0;

    'line: for line in lines {
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
                            paren_error += 1;
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
                            brace_error += 1;
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
                            bracket_error += 1;
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
                            angle_error += 1;
                            continue 'line;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // apply point values

    paren_error *= 3;
    bracket_error *= 57;
    brace_error *= 1197;
    angle_error *= 25137;

    println!("paren_error {}", paren_error);
    println!("bracket_error {}", bracket_error);
    println!("brace_error {}", brace_error);
    println!("angle_error {}", angle_error);

    println!(
        "total errors {}",
        paren_error + bracket_error + brace_error + angle_error
    );
}
