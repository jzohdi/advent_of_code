use std::fs;
const FILE_INPUT: &str = "./src/day_nine/input.txt";

pub fn solution(use_basin: bool) -> usize {
    // let mut grid:Vec<Vec<usize>> = vec![];
    let file = fs::read_to_string(FILE_INPUT).unwrap();
    let grid: Vec<Vec<usize>> = file
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|num| num.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    let mut low_points: Vec<usize> = find_low_points(&grid, use_basin);
    if use_basin {
        low_points.sort();
        let last_ele = low_points.len() - 1;
        low_points[last_ele - 2] * low_points[last_ele - 1] * low_points[last_ele]
    } else {
        low_points.into_iter().map(|point| point + 1).sum()
    }
}

fn find_low_points(grid: &Vec<Vec<usize>>, use_basin: bool) -> Vec<usize> {
    let mut low_points: Vec<usize> = vec![];
    let last_row = grid.len() - 1;

    for row in 0..=last_row {
        let row_len = grid[row].len() - 1;
        for col in 0..=row_len {
            let mut is_lowest = true;
            let curr = grid[row][col];
            // check top if applicable
            if row > 0 && curr >= grid[row - 1][col] {
                is_lowest = false;
            }
            // check right if applicable
            if col < row_len && curr >= grid[row][col + 1] {
                is_lowest = false;
            }
            // check bottom if applicable
            if row < last_row && curr >= grid[row + 1][col] {
                is_lowest = false;
            }
            // check left if applicable
            if col > 0 && curr >= grid[row][col - 1] {
                is_lowest = false;
            }
            if is_lowest {
                if use_basin {
                    low_points.push(basin_size(row, col, grid))
                } else {
                    low_points.push(curr);
                }
            }
        }
    }

    low_points
}

fn basin_size(row: usize, col: usize, grid: &Vec<Vec<usize>>) -> usize {
    let max_row = grid.len();
    let max_col = grid[0].len();
    let mut basin = 0;
    let mut stack: Vec<(usize, usize)> = vec![(row, col)];
    let mut visited = vec![vec![false; max_col]; max_row];
    visited[row][col] = true;
    while !stack.is_empty() {
        let (next_row, next_col) = stack.pop().unwrap();
        basin += 1;
        for (child_row, child_col) in get_children(next_row, next_col, max_row, max_col) {
            if grid[child_row][child_col] != 9
                && grid[child_row][child_col] > grid[next_row][next_col]
                && !visited[child_row][child_col]
            {
                stack.push((child_row, child_col));
                visited[child_row][child_col] = true;
            }
        }
    }
    basin
}

fn get_children(row: usize, col: usize, max_row: usize, max_col: usize) -> Vec<(usize, usize)> {
    let mut children: Vec<(usize, usize)> = vec![];

    if row > 0 {
        children.push((row - 1, col));
    }
    if row < max_row - 1 {
        children.push((row + 1, col));
    }
    if col > 0 {
        children.push((row, col - 1));
    }
    if col < max_col - 1 {
        children.push((row, col + 1));
    }

    children
}
