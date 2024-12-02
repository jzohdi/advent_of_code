
pub fn solution1(lines: &[String]) {
    // Iterate over each line in the file
    let mut count = 0;
    for line in lines {
        // Split the line into words and parse them into integers
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|word| word.parse().ok())
            .collect();

        // Check if the sequence is strictly increasing or decreasing
        if is_strictly_monotonic(&numbers) {
            count+=1;
            // println!("Line is strictly monotonic: {}", line);
        }
    }
    println!("day1: {}", count);
}

pub fn solution2(lines: &[String]) {
    let mut count = 0;
    for line in lines {
        // Split the line into words and parse them into integers
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|word| word.parse().ok())
            .collect();
        if is_strictly_monotonic(&numbers) {
            count+=1;
        } else {
            for i in 0..numbers.len() {
                let mut cloned_nums = numbers.clone();
                cloned_nums.remove(i);
                if is_strictly_monotonic(&cloned_nums) {
                    count+= 1;
                    break;
                }
            }
        }
        // Check if the sequence is strictly increasing or decreasing
        // if is_tolerated_monotonic(&numbers) {
        //     count+=1;
        //     // println!("Line is strictly monotonic: {}", line);
        // }
    }
    println!("day2: {}", count);
}

fn is_tolerated_monotonic(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return true; // A sequence with fewer than 2 elements is trivially monotonic
    }

    let mut increasing = true;
    let mut decreasing = true;
    
    // Convert the slice to a mutable vector
    let mut numbers_vec = numbers.to_vec();

    // Initialize an index to track the current position
    let mut i = 0;

    // Iterate while there are at least two elements to compare
    while i < numbers_vec.len() - 1 {
        let a = numbers_vec[i];
        let b = numbers_vec[i + 1];
        if a >= b {
            increasing = false;
            if !(increasing || decreasing) {
                numbers_vec.remove(i + 1);
                return is_strictly_monotonic(&numbers_vec);
            }
        }
        if a <= b {
            decreasing = false;
            if !(increasing || decreasing) {
                numbers_vec.remove(i + 1);
                return is_strictly_monotonic(&numbers_vec);
            }
        }
        let abs_val = (a - b).abs();
        if abs_val < 1 || abs_val > 3 {
            numbers_vec.remove(i + 1);
            return is_strictly_monotonic(&numbers_vec);
        }
        i+=1;
    }
    increasing || decreasing
}

// Function to check if a sequence is strictly increasing or decreasing
fn is_strictly_monotonic(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return true; // A sequence with fewer than 2 elements is trivially monotonic
    }

    let mut increasing = true;
    let mut decreasing = true;
    
    for window in numbers.windows(2) {
        if let [a, b] = window {
            if a >= b {
                increasing = false;
            }
            if a <= b {
                decreasing = false;
            }
            let abs_val = (a - b).abs();
            if abs_val < 1 || abs_val > 3 {
                return false;
            }
        }
    }

    increasing || decreasing
}