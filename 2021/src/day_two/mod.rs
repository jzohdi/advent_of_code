use std::fs;
mod position;
use position::Position;

const INPUT_FILE: &str = "./src/day_two/input.txt";

pub fn part_one() -> i32 {
    let mut position = Position::new();

    for line in file_to_commands(INPUT_FILE).lines() {
        let (command, value) = parse_command(line);
        position.move_simple(command, value);
    }
    position.distance()
}

pub fn part_two() -> i32 {
    let mut position = Position::new();

    for line in file_to_commands(INPUT_FILE).lines() {
        let (command, value) = parse_command(line);
        position.move_with_aim(command, value);
    }
    position.distance()
}

fn parse_command(line: &str) -> (&str, i32) {
    let words: Vec<&str> = line.split(" ").collect();
    (words[0], words[1].parse::<i32>().unwrap())
}

fn file_to_commands(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}
