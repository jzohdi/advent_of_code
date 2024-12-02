use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod day2 {
    pub mod solution;
}

use day2::solution::{solution1, solution2};

fn main() -> io::Result<()> {
    // Specify the path to the file'
    let file_path = "src/day2/part1.txt";
    let path = Path::new(file_path);

    // Open the file in read-only mode
    let file = File::open(&path)?;

    // Create a buffered reader for the file
    let reader = io::BufReader::new(file);
    // Collect lines into a vector
    let lines: Vec<String> = reader
    .lines() // Iterator over Result<String, io::Error>
    .collect::<Result<_, _>>()?; // Collect and handle potential errors

    solution1(&lines);
    solution2(&lines);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        println!("Hello, world!");
        let result = add(2, 3);
        assert_eq!(result, 5);
    }
}