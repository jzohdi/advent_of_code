use std::collections::HashMap;
use std::fs;

const FILE_PATH: &str = "./src/day_fourteen/input.txt";

type Counter = HashMap<char, usize>;
type Rules = HashMap<(char, char), char>;
type Polymer = HashMap<(char, char), usize>;

pub fn solution() -> () {
    println!("Day 14, part one: {}", part_one());
    println!("Day 14, part two: {}", part_two());
}

fn init_polymer(line: &str) -> Polymer {
    let mut poly = HashMap::new();

    for pairs in line.chars().collect::<Vec<char>>().windows(2) {
        poly.insert((pairs[0], pairs[1]), 1);
    }

    poly
}

fn part_two() -> usize {
    let file = fs::read_to_string(FILE_PATH).unwrap();
    let mut lines = file.lines();
    let line1 = lines.next().unwrap();
    let mut polymer: Polymer = init_polymer(line1);
    lines.next();
    let rules = extract_rules(lines.collect());

    let mut counter = count_chars(&line1.chars().collect());

    for _step in 0..10 {
        let mut new_polymer: Polymer = HashMap::new();
        for (pair, count) in &polymer.clone() {
            let rule_char = rules.get(pair).unwrap();
            let new_pair1 = (pair.0, *rule_char);
            let new_pair2 = (*rule_char, pair.1);
            *new_polymer.entry(new_pair1).or_insert(0) += count;
            *new_polymer.entry(new_pair2).or_insert(0) += count;
            *counter.entry(*rule_char).or_insert(0) += count;
        }
        polymer = new_polymer;
    }
    // println!("{:?}", counter);
    count_diff(counter, usize::MAX)
}

fn extract_rules(lines: Vec<&str>) -> Rules {
    let mut rules: HashMap<(char, char), char> = HashMap::new();
    for line in lines {
        let (pair, to) = line.split_once(" -> ").unwrap();
        let tup = (pair.chars().nth(0).unwrap(), pair.chars().nth(1).unwrap());
        rules.insert(tup, to.chars().next().unwrap());
    }
    rules
}

fn part_one() -> usize {
    let file = fs::read_to_string(FILE_PATH).unwrap();
    let mut lines = file.lines();
    let mut polymer: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    let rules = extract_rules(lines.collect());

    for _ in 0..10 {
        let mut new_poly: Vec<char> = vec![];
        for window in polymer.windows(2) {
            let key = (window[0], window[1]);
            new_poly.push(key.0);
            match rules.get(&key) {
                None => {}
                Some(&val) => new_poly.push(val),
            }
        }
        new_poly.push(*polymer.last().unwrap());
        polymer = new_poly.clone();
    }

    result(&polymer)
}

fn count_chars(chrs: &Vec<char>) -> Counter {
    let mut counter: Counter = HashMap::new();
    for &c in chrs {
        *counter.entry(c).or_insert(0) += 1;
    }
    counter
}

fn count_diff(counter: Counter, max_size: usize) -> usize {
    let mut max_count: usize = 0;
    let mut min_count: usize = max_size;
    for &count in counter.values() {
        if count > max_count {
            max_count = count;
        }
        if count < min_count {
            min_count = count;
        }
    }
    return max_count - min_count;
}

fn result(chrs: &Vec<char>) -> usize {
    let counter = count_chars(chrs);
    println!("{:?}", counter);
    count_diff(counter, chrs.len())
}
