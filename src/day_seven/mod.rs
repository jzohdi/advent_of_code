use std::cmp::{max, min};
use std::fs;

const FILE_PATH: &str = "./src/day_seven/input.txt";

pub fn solution(guassian: bool) -> usize {
    let input: Vec<usize> = fs::read_to_string(FILE_PATH)
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    (0..input.len())
        .map(|idx| calc_fuel_to_move_to(&input, idx, guassian))
        .min()
        .unwrap()
}

fn calc_fuel_to_move_to(input: &Vec<usize>, idx: usize, guassian: bool) -> usize {
    return input
        .into_iter()
        .map(|val| {
            if guassian {
                guassian_sum(min(*val, idx), max(*val, idx))
            } else {
                max(*val, idx) - min(*val, idx)
            }
        })
        .sum();
}

fn guassian_sum(lower: usize, upper: usize) -> usize {
    let n = upper - lower;
    (n * (n + 1)) / 2
}
