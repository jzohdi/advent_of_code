mod day_eight;
mod day_eleven;
mod day_five;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_ten;
mod day_three;
mod day_two;

fn main() {
    println!("Day 1, part one: {}", day_one::part_one());
    println!("Day 1, part two: {}", day_one::part_two());
    println!("Day 2, part one: {}", day_two::part_one());
    println!("Day 2, part two: {}", day_two::part_two());
    println!("Day 3, part one: {}", day_three::part_one());
    println!("Day 3, part two: {}", day_three::part_two());
    println!("Day 5, part one: {}", day_five::solution(false));
    println!("Day 5, part two: {}", day_five::solution(true));
    println!("Day 6, part one: {}", day_six::make_fish(80));
    println!("Day 6, part two: {}", day_six::make_fish(256));
    println!("Day 7, part one: {}", day_seven::solution(false));
    println!("Day 7, part two: {}", day_seven::solution(true));
    println!("Day 8, part one: {}", day_eight::solution());
    println!("Day 8, part two: {}", day_eight::solution_part_two());
    println!("Day 9, part one: {}", day_nine::solution(false));
    println!("Day 9, part two: {}", day_nine::solution(true));
    println!("Day 10, part one: {}", day_ten::part_one());
    println!("Day 10, part two: {}", day_ten::part_two());
    println!("Day 11, part one: {}", day_eleven::solution(false));
    println!("Day 11, part two: {}", day_eleven::solution(true));
}
