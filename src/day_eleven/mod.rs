use std::fs;

const FILE_INPUT: &str = "./src/day_eleven/input.txt";

pub fn solution(part_two: bool) -> usize {
    let file = fs::read_to_string(FILE_INPUT).unwrap();
    let mut grid: Vec<Vec<u8>> = file
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    let mut num_flashes = 0;

    if part_two {
        let mut step_num = 0;
        loop {
            let start_num = num_flashes;
            step_num += 1;
            inc_all_by_one(&mut grid);
            let mut has_flashed = vec![vec![false; 10]; grid.len()];
            while should_flash(&grid, &has_flashed) {
                let num = flash_step(&mut grid, &mut has_flashed);
                num_flashes += num;
            }
            if num_flashes - start_num == 100 {
                return step_num;
            }
        }
    }
    for _step in 0..100 {
        inc_all_by_one(&mut grid);
        let mut has_flashed = vec![vec![false; 10]; grid.len()];
        while should_flash(&grid, &has_flashed) {
            let num = flash_step(&mut grid, &mut has_flashed);
            num_flashes += num;
        }
        // print_grid(&grid);
    }

    num_flashes
}

fn print_grid(grid: &Vec<Vec<u8>>) {
    for row in 0..10 {
        println!("{:?}", grid[row]);
    }
    println!("");
}

fn inc_all_by_one(grid: &mut Vec<Vec<u8>>) -> () {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            grid[row][col] += 1;
        }
    }
}

fn should_flash(grid: &Vec<Vec<u8>>, has_already_flashed: &Vec<Vec<bool>>) -> bool {
    for row in 0..10 {
        for col in 0..10 {
            if grid[row][col] > 9 && !has_already_flashed[row][col] {
                return true;
            }
        }
    }
    false
}

const DIRECTIONS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn flash_step(grid: &mut Vec<Vec<u8>>, has_flashed: &mut Vec<Vec<bool>>) -> usize {
    let mut num_flashed: usize = 0;
    for row in 0..10 {
        for col in 0..10 {
            if grid[row][col] > 9 && !has_flashed[row][col] {
                for (x, y) in DIRECTIONS {
                    if is_valid_dir(row, col, x, y) {
                        let (next_row, next_col) =
                            ((row as i8 + x) as usize, (col as i8 + y) as usize);
                        if !has_flashed[next_row][next_col] {
                            grid[next_row][next_col] += 1;
                        }
                    }
                }
                num_flashed += 1;
                has_flashed[row][col] = true;
                grid[row][col] = 0;
            }
        }
    }
    num_flashed
}

fn is_valid_dir(row: usize, col: usize, x: i8, y: i8) -> bool {
    if row == 0 && x == -1 {
        false
    } else if col == 0 && y == -1 {
        false
    } else if col == 9 && y == 1 {
        false
    } else if row == 9 && x == 1 {
        false
    } else {
        true
    }
}
