use itertools::Itertools;
use std::collections::{HashMap, HashSet};

//single lowercase letter, uppercase letter, or digit. s
pub fn solution1(lines: &[String]) {
    let matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let dimensions = BoardDimensions {
        max_row: matrix.len(),
        max_col: matrix[0].len(),
    };
    let antena_map = collect_nodes(matrix);
    let edges: HashSet<Position> = antena_map
        .as_edges()
        .iter()
        .flat_map(|edge| {
            edge.valid_anti_nodes(&dimensions, |(a, b), dims| {
                vec![a.anti_node(b, dims), b.anti_node(a, dims)]
                    .into_iter()
                    .filter_map(|o| o)
                    .collect()
            })
        })
        .collect();
    println!("total: {}", edges.len());
}

struct BoardDimensions {
    max_row: usize,
    max_col: usize,
}

type AntiNodeFn = fn(antenas: (Position, Position), dims: &BoardDimensions) -> Vec<Position>;

pub fn solution2(lines: &[String]) {
    let matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let dimensions = BoardDimensions {
        max_row: matrix.len(),
        max_col: matrix[0].len(),
    };
    let antena_map = collect_nodes(matrix);
    let edges: HashSet<Position> = antena_map
        .as_edges()
        .iter()
        .flat_map(|edge| {
            edge.valid_anti_nodes(&dimensions, |(a, b), dims| {
                let mut result = vec![];
                result.append(&mut a.get_expanded_anti_nodes(b, dims));
                result.append(&mut b.get_expanded_anti_nodes(a, dims));
                return result;
            })
        })
        .collect();
    println!("total: {}", edges.len());
}

fn collect_nodes(matrix: Vec<Vec<char>>) -> AntenaMap {
    matrix
        .iter()
        .enumerate()
        .fold(AntenaMap::new(), |acc, (row, line)| acc.collect(line, row))
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Frequency(char);

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Antena {
    frequency: Frequency,
    position: Position,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

fn is_valid_node(row: isize, col: isize, dimensions: &BoardDimensions) -> bool {
    return row >= 0
        && row < dimensions.max_row as isize
        && col >= 0
        && col < dimensions.max_col as isize;
}

impl Position {
    fn anti_node(&self, other: Position, dimensions: &BoardDimensions) -> Option<Position> {
        let (diff_row, diff_col) = self.get_diff(other);
        let new_row = self.row as isize - diff_row;
        let new_col = self.col as isize - diff_col;
        if is_valid_node(new_row, new_col, dimensions) {
            Some(Position {
                row: new_row as usize,
                col: new_col as usize,
            })
        } else {
            None
        }
    }

    fn get_diff(&self, other: Position) -> (isize, isize) {
        let new_row = other.row as isize - self.row as isize;
        let new_col = other.col as isize - self.col as isize;
        return (new_row, new_col);
    }

    fn get_expanded_anti_nodes(&self, other: Position, dims: &BoardDimensions) -> Vec<Position> {
        let mut result = vec![];
        result.push(self.clone());
        let mut relative_a = self.clone();
        let mut relative_b = other.clone();
        while let Some(anti_node) = relative_a.anti_node(relative_b, dims) {
            result.push(anti_node);
            relative_b = relative_a;
            relative_a = anti_node;
        }
        return result;
    }
}
#[derive(Debug, Clone, Copy)]
struct AntenaEdge {
    nodes: (Position, Position),
}

impl AntenaEdge {
    fn from(antena_a: Antena, antena_b: Antena) -> AntenaEdge {
        AntenaEdge {
            nodes: (antena_a.position, antena_b.position),
        }
    }
    fn valid_anti_nodes(&self, dims: &BoardDimensions, generator: AntiNodeFn) -> Vec<Position> {
        generator(self.nodes, dims)
    }
}

#[derive(Debug, Clone)]
struct AntenaMap {
    antenas: HashMap<Frequency, HashSet<Antena>>,
}

impl AntenaMap {
    fn new() -> Self {
        AntenaMap {
            antenas: HashMap::new(),
        }
    }

    fn collect(self, chars: &Vec<char>, row: usize) -> Self {
        let mut new_antenas = self.antenas.clone();
        for (col, &char) in chars.iter().enumerate() {
            if char.is_alphanumeric() {
                let frequency = Frequency(char);
                let position = Position { row, col };
                let antena = Antena {
                    frequency,
                    position,
                };

                new_antenas
                    .entry(frequency)
                    .or_insert(HashSet::new())
                    .insert(antena);
            }
        }
        Self {
            antenas: new_antenas,
        }
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
