use std::cmp;
use std::collections::HashMap;
use std::fs;

const FILE_PATH: &str = "src/day_five/input.txt";

fn get_iter_coords(s_x: usize, s_y: usize, e_x: usize, e_y: usize) -> Vec<(usize, usize)> {
    if s_x == e_x {
        return (cmp::min(s_y, e_y)..=cmp::max(s_y, e_y))
            .map(|y| (s_x, y))
            .collect();
    }
    if s_y == e_y {
        return (cmp::min(s_x, e_x)..=cmp::max(s_x, e_x))
            .map(|x| (x, s_y))
            .collect();
    }
    let mut coords: Vec<(usize, usize)> = vec![];
    let mut y: usize = 0;
    if e_x > s_x {
        for x in s_x..=e_x {
            coords.push((x, (s_y as isize + (y as isize * -1)) as usize));
            y += 1;
        }
    } else {
        for x in (s_x..e_x).rev() {
            coords.push((x, s_y + y));
            y += 1;
        }
    }
    coords
}

pub fn solution(use_diagonals: bool) -> usize {
    let mut coord_map: HashMap<String, usize> = HashMap::new();
    let mut answer: usize = 0;

    let file = fs::read_to_string(FILE_PATH).unwrap();

    for line in file.lines() {
        let (start_x, start_y, end_x, end_y) = parse_line_to_coords(line);
        if use_diagonals || is_straight_line(start_x, start_y, end_x, end_y) {
            for (x, y) in get_iter_coords(start_x, start_y, end_x, end_y) {
                let key = get_map_key(x, y);
                let curr_val = coord_map.entry(key).or_insert(0);
                *curr_val += 1;
                if *curr_val == 2 {
                    answer += 1;
                }
            }
        }
        answer += 1;
    }
    answer
}

fn get_map_key(x: usize, y: usize) -> String {
    format!("{}-{}", x, y)
}

fn parse_line_to_coords(line: &str) -> (usize, usize, usize, usize) {
    let coords = line.trim().split_once(" -> ").unwrap();
    let start = coords.0.split_once(",").unwrap();
    let end = coords.1.split_once(",").unwrap();
    let s_x: usize = start.0.parse().unwrap();
    let s_y: usize = start.1.parse().unwrap();
    let e_x: usize = end.0.parse().unwrap();
    let e_y: usize = end.1.parse().unwrap();
    return (s_x, s_y, e_x, e_y);
}

fn is_straight_line(s_x: usize, s_y: usize, e_x: usize, e_y: usize) -> bool {
    s_x == e_x || s_y == e_y
}
