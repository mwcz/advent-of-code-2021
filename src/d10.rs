//! A solution to day 10 year 2021.
//! https://adventofcode.com/2021/day/10

type Model = String;
type Answer = u64;

pub fn parse(input: String) -> Model {
    input
}

pub fn part1(input: Model) -> Answer {
    let mut stack: Vec<char> = Vec::new();

    let mut paren_stack = 0;
    let mut paren_error = 0;

    let mut brace_stack = 0;
    let mut brace_error = 0;

    let mut bracket_stack = 0;
    let mut bracket_error = 0;

    let mut angle_stack = 0;
    let mut angle_error = 0;

    'line: for line in input.lines() {
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

    // apply point values

    paren_error *= 3;
    bracket_error *= 57;
    brace_error *= 1197;
    angle_error *= 25137;

    paren_error + bracket_error + brace_error + angle_error
}

pub fn part2(input: Model) -> Answer {
    let mut stack: Vec<char> = Vec::new();

    let mut scores: Vec<u64> = Vec::new();

    let mut paren_stack = 0;
    let mut brace_stack = 0;
    let mut bracket_stack = 0;
    let mut angle_stack = 0;

    'line: for line in input.lines() {
        stack.clear();
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

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d10");

    #[test]
    fn d10p1_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 296535);
    }

    #[test]
    fn d10p2_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 4245130838);
    }
}
