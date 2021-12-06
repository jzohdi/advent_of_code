use std::fs;

const FILE_PATH: &str = "./src/day_six/input.txt";

pub fn make_fish(num_days: usize) -> usize {
    let input_line: Vec<u8> = read_input_line(FILE_PATH);

    let mut current_state = vec![0; 9];

    input_line.into_iter().for_each(|x| {
        current_state[x as usize] += 1;
    });

    for _ in 0..num_days {
        let num_will_spawn = current_state[0];
        for fish_num in 1..9 {
            current_state[fish_num - 1] = current_state[fish_num];
        }
        current_state[8] = num_will_spawn;
        current_state[6] += num_will_spawn;
    }

    return current_state.iter().sum();
}

fn read_input_line(file_path: &str) -> Vec<u8> {
    let file_str = fs::read_to_string(file_path).unwrap();
    let first_line = file_str.lines().next().unwrap();
    return first_line
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
}
