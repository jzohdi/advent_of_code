use std::collections::HashMap;
use std::fs;
const FILE_PATH: &str = "./src/day_ten/input.txt";

pub fn part_one() -> usize {
    let file = fs::read_to_string(FILE_PATH).unwrap();

    let closers: HashMap<char, usize> = vec![(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .into_iter()
        .collect();
    let matchers: HashMap<char, char> = vec![(')', '('), (']', '['), ('}', '{'), ('>', '<')]
        .into_iter()
        .collect();
    let mut score: usize = 0;

    for line in file.lines() {
        let mut stack: Vec<char> = vec![];
        for c in line.trim().chars() {
            // if it is a closer
            if let Some(value) = closers.get(&c) {
                if stack.len() < 1 {
                    score += value;
                    break;
                } else {
                    let last_char = stack.pop().unwrap();
                    // if the last char in the stack
                    if last_char != *matchers.get(&c).unwrap() {
                        score += value;
                        break;
                    }
                }
            // else is an opener, so append to stack
            } else {
                stack.push(c);
            }
        }
    }

    score
}

pub fn part_two() -> usize {
    let file = fs::read_to_string(FILE_PATH).unwrap();

    let closers: HashMap<char, usize> = vec![(')', 1), (']', 2), ('}', 3), ('>', 4)]
        .into_iter()
        .collect();
    let matchers: HashMap<char, char> = vec![('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .into_iter()
        .collect();
    let match_closer: HashMap<char, char> = vec![(')', '('), (']', '['), ('}', '{'), ('>', '<')]
        .into_iter()
        .collect();
    let mut scores: Vec<usize> = vec![];

    for line in file.lines() {
        let mut stack: Vec<char> = vec![];
        let mut is_corrupted = false;
        for c in line.trim().chars() {
            // if it is a closer, pop
            if let Some(_) = closers.get(&c) {
                if stack.len() == 0 {
                    is_corrupted = true;
                } else {
                    let last_char = stack.pop().unwrap();
                    if last_char != *match_closer.get(&c).unwrap() {
                        is_corrupted = true;
                        break;
                    }
                }
            // else is an opener, so append to stack
            } else {
                stack.push(c);
            }
        }
        if is_corrupted {
            continue;
        }
        let mut score = 0;
        stack.reverse();
        for c in stack {
            let closer = matchers.get(&c).unwrap();
            let value = closers.get(&closer).unwrap();
            score *= 5;
            score += value;
        }
        scores.push(score);
    }
    scores.sort();
    scores[scores.len() / 2]
}
