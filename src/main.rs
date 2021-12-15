use std::env;

mod day_eight;
mod day_eleven;
mod day_five;
mod day_fourteen;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_ten;
mod day_thirteen;
mod day_three;
mod day_twelve;
mod day_two;

fn main() {
    // let days: Vec<String> = (0..15).map(|d| d.to_string()).collect();
    let cmd_args: Vec<String> = env::args().collect();
    if cmd_args.contains(&String::from("1")) {
        println!("Day 1, part one: {}", day_one::part_one());
        println!("Day 1, part two: {}", day_one::part_two());
    }
    if cmd_args.contains(&String::from("2")) {
        println!("Day 2, part one: {}", day_two::part_one());
        println!("Day 2, part two: {}", day_two::part_two());
    }
    if cmd_args.contains(&String::from("3")) {
        println!("Day 3, part one: {}", day_three::part_one());
        println!("Day 3, part two: {}", day_three::part_two());
    }
    if cmd_args.contains(&String::from("5")) {
        println!("Day 5, part one: {}", day_five::solution(false));
        println!("Day 5, part two: {}", day_five::solution(true));
    }
    if cmd_args.contains(&String::from("6")) {
        println!("Day 6, part one: {}", day_six::make_fish(80));
        println!("Day 6, part two: {}", day_six::make_fish(256));
    }
    if cmd_args.contains(&String::from("7")) {
        println!("Day 7, part one: {}", day_seven::solution(false));
        println!("Day 7, part two: {}", day_seven::solution(true));
    }
    if cmd_args.contains(&String::from("8")) {
        println!("Day 8, part one: {}", day_eight::solution());
        println!("Day 8, part two: {}", day_eight::solution_part_two());
    }
    if cmd_args.contains(&String::from("9")) {
        println!("Day 9, part one: {}", day_nine::solution(false));
        println!("Day 9, part two: {}", day_nine::solution(true));
    }
    if cmd_args.contains(&String::from("10")) {
        println!("Day 10, part one: {}", day_ten::part_one());
        println!("Day 10, part two: {}", day_ten::part_two());
    }
    if cmd_args.contains(&String::from("11")) {
        println!("Day 11, part one: {}", day_eleven::solution(false));
        println!("Day 11, part two: {}", day_eleven::solution(true));
    }
    if cmd_args.contains(&String::from("12")) {
        println!("Day 12, part one: {}", day_twelve::part_one());
        println!("Day 12, part two: {}", day_twelve::part_two());
    }
    if cmd_args.contains(&String::from("13")) {
        println!("Day 13, part one: {}", day_thirteen::part_one());
        day_thirteen::part_two();
    }
    if cmd_args.contains(&String::from("14")) {
        day_fourteen::solution();
    }
}
