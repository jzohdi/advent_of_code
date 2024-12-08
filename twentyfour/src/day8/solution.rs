use std::collections::{HashMap, HashSet};
use itertools::Itertools;

//single lowercase letter, uppercase letter, or digit. s
pub fn solution1(lines: &[String]) {
    let matrix: Vec<Vec<char>> = lines.iter().map(|line| {
        line.chars().collect()
    }).collect();
    let antena_map = collect_nodes(matrix);
    let edges = antena_map.as_edges();
    // println!("map: {:?}", antena_map);
}

pub fn solution2(lines: &[String]) {
    println!("{}", lines.len());
}


fn collect_nodes(matrix: Vec<Vec<char>>) -> AntenaMap {
    matrix.iter().enumerate().fold(AntenaMap::new(), |acc, (row, line)| {
        acc.collect(line, row)
    })
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Frequency(char);

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Antena {
    frequency: Frequency,
    position: Position
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Position {
    row: usize,
    col: usize
}

#[derive(Debug, Clone, Copy)]
struct AntenaEdge {
    antenas: (Antena, Antena),
    anti_nodes: (Position, Position)
}

impl AntenaEdge {
    fn from(antena_a: Antena, antena_b: Antena) -> AntenaEdge {

    }
}

#[derive(Debug, Clone)]
struct AntenaMap {
    antenas: HashMap<Frequency, HashSet<Antena>>
}

impl AntenaMap {
    fn new() -> Self {
        AntenaMap {
            antenas: HashMap::new()
        }
    }

    fn collect(self, chars: &Vec<char>, row: usize) -> Self {
        let mut new_antenas = self.antenas.clone();
        for (col, &char) in chars.iter().enumerate() {
            if char.is_alphanumeric() {
                let frequency = Frequency(char);
                let position = Position { row, col };
                let antena = Antena { frequency, position };
                
                new_antenas.entry(frequency)
                    .or_insert(HashSet::new())
                    .insert(antena);
            }
        }
        Self { antenas: new_antenas }
    }

    fn as_edges(self) -> Vec<AntenaEdge> {
        self.antenas.keys().fold(Vec::new(), |acc, key| {
            let values = self.antenas.get(&key).unwrap();
            let result = values
                .iter()
                .cloned()
                .combinations(2)
                .fold(acc, |acc, antenas| {
                    let mut acc = acc.clone();
                     acc.push(AntenaEdge::from(antenas[0], antenas[1]));
                    return acc;
                });
            return result;
        })

    }
}