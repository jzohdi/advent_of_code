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
        poly.entry((pairs[0], pairs[1]))
            .and_modify(|c| *c += 1)
            .or_insert(1);
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

    for _step in 0..40 {
        let mut new_polymer: Polymer = HashMap::new();
        for (pair, count) in &polymer.clone() {
            let rule_char = rules.get(pair).unwrap();
            let (new_pair1, new_pair2) = ((pair.0, *rule_char), (*rule_char, pair.1));
            new_polymer
                .entry(new_pair1)
                .and_modify(|c| *c += count)
                .or_insert(*count);
            new_polymer
                .entry(new_pair2)
                .and_modify(|c| *c += count)
                .or_insert(*count);
            counter
                .entry(*rule_char)
                .and_modify(|c| *c += count)
                .or_insert(*count);
        }
        polymer = new_polymer;
    }
    count_diff(counter)
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
    chrs.into_iter().fold(HashMap::new(), |mut acc, &curr| {
        acc.entry(curr).and_modify(|c| *c += 1).or_insert(1);
        acc
    })
}

fn count_diff(counter: Counter) -> usize {
    let max_count: usize = *counter.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let min_count: usize = *counter.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    return max_count - min_count;
}

fn result(chrs: &Vec<char>) -> usize {
    count_diff(count_chars(chrs))
}
