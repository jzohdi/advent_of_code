use std::collections::{HashMap, HashSet};
use std::fs;

const FILE_PATH: &str = "./src/day_thirteen/input.txt";

type Grid = HashMap<usize, HashSet<usize>>;

#[derive(Debug)]
struct Paper<'a> {
    points: Grid,
    len_x: usize,
    len_y: usize,
    instructions: Vec<&'a str>,
}

impl<'a> Paper<'a> {
    pub fn new() -> Paper<'a> {
        Paper {
            points: HashMap::new(),
            len_x: 0,
            len_y: 0,
            instructions: vec![],
        }
    }
    pub fn count_points(&self) -> usize {
        let mut count = 0;
        for y in self.points.keys() {
            count += self.points.get(y).unwrap().len();
        }
        count
    }

    pub fn fold_x(&mut self, value: usize) -> () {
        for row in self.points.values_mut() {
            // let row = self.points.get_mut(y).unwrap();
            for x in (value + 1)..(value * 2 + 1) {
                if row.contains(&x) {
                    let new_value = (value * 2) - x;
                    row.insert(new_value);
                    row.remove(&x);
                }
            }
        }
        self.len_x = value;
    }

    pub fn fold_y(&mut self, value: usize) -> () {
        for y in (value + 1)..((value * 2) + 1) {
            if self.points.contains_key(&y) {
                let dest_row = (value * 2) - y;
                let x_vals = self.points.remove(&y).unwrap();
                // let mut dest_row = HashSet::new();
                if !self.points.contains_key(&dest_row) {
                    self.points.insert(dest_row, x_vals);
                } else {
                    let dest_set = self.points.get_mut(&dest_row).unwrap();
                    dest_set.extend(&x_vals);
                }
            };
        }
        self.len_y = value;
    }

    pub fn print(&self) -> () {
        for y in 0..self.len_y {
            if !self.points.contains_key(&y) {
                println!("{:.<1$}", "", self.len_x);
            } else {
                for x in 0..self.len_x {
                    if self.points.get(&y).unwrap().contains(&x) {
                        print!("#")
                    } else {
                        print!(".")
                    }
                }
            }
            print!("\n")
        }
    }
}

pub fn part_one() -> usize {
    let file = fs::read_to_string(FILE_PATH).unwrap();
    let mut paper = make_grid(file.lines().map(|l| l.trim()).collect());

    let (dir, value) = parse_instruction(paper.instructions[0]);
    if dir == "x" {
        paper.fold_x(value);
    } else {
        paper.fold_y(value);
    }

    return paper.count_points();
}

pub fn part_two() -> () {
    let file = fs::read_to_string(FILE_PATH).unwrap();
    let mut paper = make_grid(file.lines().map(|l| l.trim()).collect());

    for instruction in paper.instructions.clone() {
        let (dir, value) = parse_instruction(instruction);
        if dir == "x" {
            paper.fold_x(value);
        } else {
            paper.fold_y(value);
        }
    }
    paper.print();
}

fn parse_instruction(instruction: &str) -> (&str, usize) {
    let (dir, value) = instruction.split_once("=").unwrap();
    (dir, value.parse().unwrap())
}

fn make_grid(lines: Vec<&str>) -> Paper {
    let mut paper = Paper::new();

    for line in lines {
        if line.contains(",") {
            let (x, y) = line.split_once(",").unwrap();
            let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
            paper.points.entry(y).or_insert(HashSet::new()).insert(x);
            if x >= paper.len_x {
                paper.len_x = x + 1
            }
            if y >= paper.len_y {
                paper.len_y = y + 1;
            }
        } else if line.contains("=") {
            paper
                .instructions
                .push(line.split_whitespace().last().unwrap());
        };
    }
    return paper;
}
