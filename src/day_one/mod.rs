use std::fs;

pub fn part_one() -> i32 {
    return file_to_ints("./src/day_one/input.txt")
        .into_iter()
        .fold((0, None), |(acc, prev), curr| {
            if let Some(p) = prev {
                if p < curr {
                    return (acc + 1, Some(curr));
                }
            }
            (acc, Some(curr))
        })
        .0;
}

pub fn part_two() -> i32 {
    let nums = file_to_ints("./src/day_one/input.txt");

    return (3..nums.len()).fold(0, |acc, idx| {
        if nums[idx - 2] + nums[idx - 1] + nums[idx] > nums[idx - 3] + nums[idx - 2] + nums[idx - 1]
        {
            acc + 1
        } else {
            acc
        }
    });
}

fn file_to_ints(file: &str) -> Vec<i32> {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}
