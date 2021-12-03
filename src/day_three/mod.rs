use std::fs;
mod binary_parser;
use binary_parser::BinaryParser;

const INPUT_FILE: &str = "./src/day_three/input.txt";

pub fn part_one() -> usize {
    let mut parser = BinaryParser::new();

    let file = fs::read_to_string(INPUT_FILE).unwrap();

    for line in file.lines() {
        parser.parse_line(line);
    }

    binary_parser::PowerCalculator::power_consumption(
        parser.most_common_bits(),
        parser.lest_common_bits(),
    )
}
/*

use first bit
- keep ony numbers select by bit criteria
- if only one number left => answer
- otherwise repeat using second bit

*/
fn filter_lines(lines: Vec<&str>, bits: Vec<u8>, current_idx: usize) -> Vec<&str> {
    let mut new_lines = vec![];
    for line in lines {
        let bit = line.chars().nth(current_idx).unwrap();
        match bit {
            '0' => {
                if bits[current_idx] == 0 {
                    new_lines.push(line);
                }
            }
            '1' => {
                if bits[current_idx] == 1 {
                    new_lines.push(line)
                }
            }
            _ => panic!("Should not happen"),
        }
    }
    return new_lines;
}

pub fn part_two() -> usize {
    let file = fs::read_to_string(INPUT_FILE).unwrap();

    let mut current_idx = 0;
    let mut oxygen: Vec<&str> = vec![];
    let mut co2: Vec<&str> = vec![];

    for line in file.lines() {
        oxygen.push(line);
        co2.push(line);
    }
    let mut lcb = vec![];
    let mut mcb = vec![];
    loop {
        if oxygen.len() > 1 {
            let mut oxy_counter = BinaryParser::new();
            for line in oxygen.clone() {
                oxy_counter.parse_line(line);
            }
            mcb = oxy_counter.most_common_bits();
            oxygen = filter_lines(oxygen.clone(), mcb.clone(), current_idx);
        }
        if co2.len() > 1 {
            let mut co2_counter = BinaryParser::new();
            for line in co2.clone() {
                co2_counter.parse_line(line);
            }
            lcb = co2_counter.lest_common_bits();
            co2 = filter_lines(co2.clone(), lcb.clone(), current_idx);
        }
        if co2.len() < 2 && oxygen.len() < 2 {
            return binary_parser::PowerCalculator::power_consumption(mcb, lcb);
        }
        current_idx += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut current_idx = 0;
        let mut oxygen = vec![
            "011110110110",
            "011110110011",
            "011110110001",
            "011110110101",
        ];
        loop {
            if oxygen.len() > 1 {
                let mut oxy_counter = BinaryParser::new();
                for line in oxygen.clone() {
                    oxy_counter.parse_line(line);
                }
                let mcb = oxy_counter.most_common_bits();
                println!(
                    "curr oxy: {:?}\n from bits: {:?}\n, currentidx: {}",
                    oxygen, mcb, current_idx
                );
                oxygen = filter_lines(oxygen.clone(), mcb, current_idx);
                println!("oxygen after: {:?}\n", oxygen);
            }
            if oxygen.len() < 2 {
                assert_eq!(1974, usize::from_str_radix(oxygen[0], 2).unwrap());
                break;
            }
            current_idx += 1
        }
    }
}
