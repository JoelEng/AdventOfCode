use std::collections::HashMap;

const OPENERS: [char; 4] = ['(', '[', '{', '<'];

#[aors::main]
fn main(input: &str) -> (i32, u128) {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    solve(input)
}

fn solve(input: Vec<Vec<char>>) -> (i32, u128) {
    let mut illegal_char_score = 0;
    let mut incomplete_score;
    let mut corrupted;
    let mut incomplete_lines: Vec<u128> = vec![];

    let mut map = HashMap::new();
    map.insert(')', '(');
    map.insert(']', '[');
    map.insert('}', '{');
    map.insert('>', '<');

    for line in input {
        let mut stack: Vec<char> = vec![];
        corrupted = false;
        for &c in &line {
            if OPENERS.contains(&c) {
                stack.push(c);
            } else {
                if map.entry(c).or_default() == stack.last().unwrap() {
                    stack.pop();
                } else {
                    match c {
                        ')' => illegal_char_score += 3,
                        ']' => illegal_char_score += 57,
                        '}' => illegal_char_score += 1197,
                        '>' => illegal_char_score += 25137,
                        _ => (),
                    }
                    corrupted = true;
                    break;
                }
            }
        }
        if !corrupted {
            incomplete_score = 0;
            for c in stack.iter().rev() {
                incomplete_score *= 5;
                incomplete_score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                }
            }
            incomplete_lines.push(incomplete_score);
        }
    }

    let p1 = illegal_char_score;
    incomplete_lines.sort();
    let middle = incomplete_lines.len() / 2;
    (p1, incomplete_lines[middle])
}
