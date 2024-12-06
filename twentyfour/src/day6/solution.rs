use std::collections::HashMap;

pub fn solution1(lines: &[String]) {  
    let mut matrix: Vec<Vec<char>> = lines.iter()
        .map(|line| line.chars().collect())
        .collect();
    let mut map = HashMap::new();
    map.insert('^', (-1, 0, '>'));
    map.insert('>', (0, 1, 'v'));
    map.insert('v', (1, 0, '<'));
    map.insert('<', (0, -1, '^'));
    if let Some((row, col)) = find_character(lines, &map) {
        let num_filled =  1 + fill_matrix_from(&mut matrix, map, row as i32, col as i32);
        println!("num filled: {}", num_filled);
    }
    // println!("{:?}", matrix);
}

pub fn solution2(lines: &[String]) {  
    println!("{}", lines.len());
}

fn fill_matrix_from(matrix: &mut Vec<Vec<char>>, guard: HashMap<char, (i32, i32, char)>, row: i32, col: i32) -> i32 {
    let cur_guard_char = matrix[row as usize][col as usize];
    // let num_filled = 1;
    let current_guard = guard.get(&cur_guard_char);
    let (dir_row, dir_col, next_guard) = current_guard.unwrap();
    let next_row = get_next(row, dir_row);
    let next_col = get_next(col, dir_col);
    // after retreiving details on the current pos, set current to 'X'
    matrix[row as usize][col as usize] = 'X';
    if is_valid_next(next_row, next_col, matrix) {
        let next_char = matrix[next_row as usize][next_col as usize];
        // if the next char is '#', then get the next right turn guard and place in the next position;
        if next_char == '#' {
            let (dir_row, dir_col, _guard) = guard.get(next_guard).unwrap();
            let next_row = get_next(row, dir_row);
            let next_col = get_next(col, dir_col);
            if !is_valid_next(next_row, next_col, matrix) {
                return 0;
            }
            // set the next position as the next guard;
            let curr_next =matrix[next_row as usize][next_col as usize];
            matrix[next_row as usize][next_col as usize] = *next_guard;
            // if we are filling an already filled next don't count + 1;
            if curr_next == 'X' {
                return fill_matrix_from(matrix, guard, next_row, next_col);
            }
            return 1 + fill_matrix_from(matrix, guard, next_row, next_col);
        } 
        // set the next position as the next guard;
        let curr_next =matrix[next_row as usize][next_col as usize];
        matrix[next_row as usize][next_col as usize] = cur_guard_char;
        if curr_next == 'X' {
            return fill_matrix_from(matrix, guard, next_row, next_col);
        }
        return 1 + fill_matrix_from(matrix, guard, next_row, next_col);        
    }
    return 0;
}

fn get_next(pos_index: i32, dir: &i32) -> i32 {
    return pos_index + *dir;
}

fn is_valid_next(row: i32, col: i32, matrix: &Vec<Vec<char>>) -> bool {
    return row >= 0 && col >= 0 && row < matrix.len() as i32 && col < matrix[0].len() as i32;
}


fn find_character(matrix: &[String], targets: &HashMap<char, (i32, i32, char)>) -> Option<(usize, usize)> {
    for (row_index, row) in matrix.iter().enumerate() {
        if let Some(col_index) = row.chars().position(|c| targets.contains_key(&c)) {
            return Some((row_index, col_index));
        }
    }
    None
}

