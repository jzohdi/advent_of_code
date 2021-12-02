use std::fs;

pub fn answer() {
    println!(
        "Answer: {}",
        fs::read_to_string("./src/day_one/input.txt")
            .unwrap()
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .fold((0, None), |(acc, prev), curr| {
                if let Some(p) = prev {
                    if p > curr {
                        return (acc + 1, Some(curr));
                    }
                }
                (acc, Some(curr))
            })
            .0
    )
}
