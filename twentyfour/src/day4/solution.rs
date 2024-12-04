// use std::collections::HashSet;

pub fn solution1(lines: &[String]) {    
    let mut total = 0;

    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            // check to see if this is a valid starting point
            let letter = lines[row].chars().nth(col).unwrap();
            if !is_correct_char_at(letter, 0) {
                continue
            }
            // need to know which direction we are going
            let next_neighbors = get_neighbors(row, col, lines.len(), lines[row].len());
            for (delta_row, delta_col) in next_neighbors {
                let next_row = row.checked_add_signed(delta_row).unwrap();
                let next_col = col.checked_add_signed(delta_col).unwrap();
                if found_xmas_in_direction(lines, next_row, next_col, 1, delta_row, delta_col, 1, 4) {
                    total += 1;
                }
            }
        }
    }

    println!("total found: {}", total);
}

pub fn solution2(lines: &[String]) {    
    let mut total = 0;
    // let max_row = lines.len();
    // let max_col = lines[0].len();
    // let visited: HashSet<String> = HashSet::new();
    let next_starts = [
        ((0, 2), (1, -1)), // check same row, 2 over going down left
        ((2, 0), (-1, 1))]; // check 2 down, same col going up right  
    let next_t_starts = [
        ((0, 1), (1, 0)),
        ((1, 0), (0, 1))
    ];
    // let deltas = (1, 1); // going down right
    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            // check to see if this is a valid starting point
            let letter = lines[row].chars().nth(col).unwrap();
            // if the char is M then check for AS. then check for MAS in the two other possible dirs
            if is_correct_char_at(letter, 1) && makes_rest(row, col, lines, 3) {
                total += find_match(row, col, lines, next_starts, 1, 4);
            } // if the char is S then check for AM. then check for SAM in the two other possible places
            else if is_correct_char_at(letter, 3) && makes_rest(row, col, lines, 1) {
                total += find_match(row, col, lines, next_starts, -1, 0);
            } else if is_correct_char_at(letter, 1) && makes_rest_down(row, col, lines, 3) {
                total += find_match(row, col, lines, next_t_starts, 1, 4);
            } else if is_correct_char_at(letter, 1) && makes_rest_down(row, col, lines, 1) {
                total += find_match(row, col, lines, next_t_starts, -1, 0);
            }
        }
    }

    println!("total found: {}", total);
}

fn find_match(row: usize, col: usize, lines:&[String], next_starts: [((isize, isize), (isize, isize)); 2], direction: i32, stop_index: i32) -> i32 {
    let max_row = lines.len();
    let max_col = lines[0].len();
    for ((delta_start_row, delta_start_col), (delta_row, delta_col)) in next_starts {
        if !is_valid_next(row, col, delta_start_row, delta_start_col, max_row, max_col) {
            continue;
        }
        let next_row = row.checked_add_signed(delta_start_row).unwrap();
        let next_col = col.checked_add_signed(delta_start_col).unwrap();
        // if one is valid, then the other cannot be
        if found_xmas_in_direction(lines, next_row, next_col, 3, delta_row, delta_col, direction, stop_index)  {
            return 1;
        }
    }
    return 0;
}

fn makes_rest_down(row: usize, col: usize, lines: &[String], last_char: i32) -> bool { 
    if !is_valid_next(row, col, 1, 0, lines.len(), lines[row].len()) {
        return false;
    }
    if !is_valid_next(row, col, 2, 0, lines.len(), lines[row].len()) {
        return false;
    }
    let mut next_row = row + 1;
    let mut next_col = col;
    let mut letter = lines[next_row].chars().nth(next_col).unwrap();
    if !is_correct_char_at(letter, 2) {
        return false;
    }
    next_row += 1;
    letter = lines[next_row].chars().nth(next_col).unwrap();
    if !is_correct_char_at(letter, last_char) {
        return false;
    }
    return true;
}

fn makes_rest(row: usize, col: usize, lines: &[String], last_char: i32) -> bool {
    if !is_valid_next(row, col, 1, 1, lines.len(), lines[row].len()) {
        return false;
    }
    if !is_valid_next(row, col, 2, 2, lines.len(), lines[row].len()) {
        return false;
    }
    let mut next_row = row + 1;
    let mut next_col = col + 1;
    let mut letter = lines[next_row].chars().nth(next_col).unwrap();
    if !is_correct_char_at(letter, 2) {
        return false;
    }
    next_row += 1;
    next_col += 1;
    letter = lines[next_row].chars().nth(next_col).unwrap();
    if !is_correct_char_at(letter, last_char) {
        return false;
    }
    return true;
}

// I know that I'm iterating to the left and then down. 
// So don't need to check up or to the left
// fn get_mas_starts(row: usize, col: usize, delta_row: isize, delta_col: isize) {
//     let right_start = (0 ,2); // same row, 2 cols over
//     let down_start = (2, 0);
// }

fn found_xmas_in_direction(lines: &[String], row: usize, col: usize, index: i32, delta_row: isize, delta_col: isize, direction: i32, stop_index: i32) -> bool {
    let char = lines[row].chars().nth(col);
    if direction > 0 && index >= stop_index {
        return false;
    }
    if direction < 0 && index <= stop_index  {
        return false;
    }
    let max_row = lines.len();
    let max_col = lines[row].len();
    match char {
        Some(letter) => {
            if !is_correct_char_at(letter, index) {
                return false;
            } 
            if index == 3 {
                // println!("found xmas at {} {}. letter: {}, index: {}", row, col, letter, index);
                return true;
            }
            if !is_valid_next(row, col, delta_row, delta_col, max_row, max_col) {
                return false;
            }
            let next_row = row.checked_add_signed(delta_row).unwrap();
            let next_col = col.checked_add_signed(delta_col).unwrap();
            if found_xmas_in_direction(lines, next_row, next_col, index + direction, delta_row, delta_col, direction, stop_index) {
                return true;
            }
            return false;
        },
        None => return false
    }
}

fn is_correct_char_at(letter: char, index: i32) -> bool {
  match index {
      0 => return letter == 'X',
      1 => return letter == 'M',
      2 => return letter == 'A',
      3 => return letter == 'S',
      _ => return false
  }
}

fn get_neighbors(row: usize, col: usize, max_row: usize, max_col: usize) -> Vec<(isize, isize)> {
    let mut neighbors = Vec::new();

    let deltas = [
        (-1, -1), (-1, 0), (-1, 1), 
        (0, -1),         (0, 1),    
        (1, -1), (1, 0), (1, 1),    
    ];

    for &(delta_row, delta_col) in &deltas {
        // Check if the new coordinates are within grid bounds
        if is_valid_next(row, col, delta_row, delta_col, max_row, max_col) {
            neighbors.push((delta_row, delta_col));
        }
    }

    neighbors
}

fn is_valid_next(row: usize, col: usize, delta_row: isize, delta_col: isize, max_row: usize, max_col: usize) -> bool {
    let new_row = row as isize + delta_row;
    let new_col = col as isize + delta_col;
    return new_row >= 0 && new_row < max_row as isize && new_col >= 0 && new_col < max_col as isize
}