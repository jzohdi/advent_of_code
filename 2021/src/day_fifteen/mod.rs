use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solution() -> () {
    // println!("{:?}", grid);
    run(false);
    run(true);
}

fn run(expand: bool) -> () {
    let file = fs::read_to_string("./src/day_fifteen/input.txt").unwrap();
    let mut grid: Vec<Vec<usize>> = file
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    if expand {
        expand_grid(&mut grid);
        println!("{:?}", grid);
    }
    let last_row = grid.len() - 1;
    let last_col = grid[0].len() - 1;
    let mut costs = vec![vec![usize::MAX; last_col + 1]; last_row + 1];
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    costs[0][0] = 0;
    heap.push(State {
        cost: 0,
        pos: (0, 0),
    });

    while let Some(State { cost, pos }) = heap.pop() {
        let (row, col) = pos;
        if row == last_row && col == last_col {
            println!("cost: {}", cost);
            return;
        }

        for dir in get_neighbors(row, col, last_row, last_col) {
            let (next_row, next_col) = dir;
            let next_state = State {
                cost: cost + grid[next_row][next_col],
                pos: (next_row, next_col),
            };

            if next_state.cost < costs[next_row][next_col] {
                heap.push(next_state);
                costs[next_row][next_col] = next_state.cost;
            }
        }
    }
}

fn get_neighbors(row: usize, col: usize, last_row: usize, last_col: usize) -> Vec<(usize, usize)> {
    let mut n = vec![];
    if row > 0 {
        n.push((row - 1, col));
    }
    if row < last_row {
        n.push((row + 1, col));
    }
    if col > 0 {
        n.push((row, col - 1));
    }
    if col < last_col {
        n.push((row, col + 1));
    }
    n
}

fn expand_grid(grid: &mut Vec<Vec<usize>>) {
    // expand the first rows to be 5 x wide
    for j in 0..grid.len() {
        let mut new_row = vec![grid[j].clone()];
        for last_row in 0..5 {
            let new_cols = new_row[last_row]
                .iter()
                .map(|&n| if n < 9 { n + 1 } else { 1 })
                .collect();
            new_row.push(new_cols);
        }

        for row in 1..5 {
            grid[j].append(&mut new_row[row]);
        }
    }
    // expand each of the rows 5 x down
    let mut last_rows = grid.clone();
    for _ in 0..4 {
        let mut new_rows: Vec<Vec<usize>> = vec![];
        for row in &last_rows {
            new_rows.push(row.iter().map(|&x| if x < 9 { x + 1 } else { 1 }).collect());
        }

        for row in new_rows.iter() {
            grid.push(row.clone());
        }
        last_rows = new_rows;
    }
}
