use std::collections::HashMap;
use std::fs;
use cached::proc_macro::cached;

const FILE_PATH: &str = "./src/day_fourteen/input.txt";
static RULES: Rules = HashMap::new();
static polymer: Vec<char> = Vec::new();
type Counter = HashMap<char, usize>;
type Rules = HashMap<(char, char), char>;

pub fn solution() {
    load_file();
    println!("Day 14, part one: {}", part_one());
    println!("Day 14, part two: {}", part_two());
}

fn load_file() {
    let file = fs::read_to_string(FILE_PATH).unwrap();
    let mut lines = file.lines();
    
    lines.next().unwrap().chars().for_each(|c| {
        polymer.push(c);
    })
    
    lines.next();

    for line in lines {
        let (pair, to) = line.split_once(" -> ").unwrap();
        let tup = (pair.chars().nth(0).unwrap(), pair.chars().nth(1).unwrap());
        RULES.insert(tup, to.chars().next().unwrap());
    }


} 

fn part_two() -> usize {
    let mut counter: HashMap<char, usize> = HashMap::new();

    for c in polymer.clone() {
        let count = *counter.entry(c).or_insert(0) + 1;
        counter.insert(c, count);
    }

    for pair in polymer.windows(2) {
        // println!("expanding pair: {:?}", pair);
        let pair = (pair[0], pair[1]);
        expand(pair, 0);
        // expand(pair, 0, &mut counter, &rules);
    }

    let mut max_count: usize = 0;
    let mut min_count: usize = usize::max_value();
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

#[cached]
fn expand(pair: (char, char), curr_step: usize) -> () {//, counter: &mut Counter, rules: &Rules) -> () {
    if curr_step >= 20 {
        return;
    }
    match RULES.get(&pair) {
        None => {}
        Some(&c) => {
            // let new_count = *counter.entry(c).or_insert(0) + 1;
            // counter.insert(c, new_count);
            expand((pair.0, c), curr_step + 1); //, counter, rules);
            expand((c, pair.1), curr_step + 1);//, counter, rules);
        }
    }
}

fn part_one() -> usize {
    // let file = fs::read_to_string(FILE_PATH).unwrap();
    // let mut lines = file.lines();
    // let mut polymer: Vec<char> = lines.next().unwrap().chars().collect();
    // let mut rules: HashMap<(char, char), char> = HashMap::new();

    // lines.next();

    // for line in lines {
    //     let (pair, to) = line.split_once(" -> ").unwrap();
    //     let tup = (pair.chars().nth(0).unwrap(), pair.chars().nth(1).unwrap());
    //     rules.insert(tup, to.chars().next().unwrap());
    // }

    for _ in 0..10 {
        let mut new_poly: Vec<char> = vec![];
        for window in polymer.windows(2) {
            let key = (window[0], window[1]);
            new_poly.push(key.0);
            match RULES.get(&key) {
                None => {}
                Some(&val) => new_poly.push(val),
            }
        }
        // println!("new poly: {:?}", new_poly);
        new_poly.push(*polymer.last().unwrap());
        polymer = new_poly.clone();
    }

    result(&polymer)
}

fn result(chrs: &Vec<char>) -> usize {
    let mut counter: Counter = HashMap::new();
    let mut max_count: usize = 0;
    let mut min_count: usize = chrs.len();
    for &c in chrs {
        let count = *counter.entry(c).or_insert(0) + 1;
        counter.insert(c, count);
    }
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
