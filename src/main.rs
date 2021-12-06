mod day_one;
mod day_six;
mod day_three;
mod day_two;

fn main() {
    println!("Day 1, part one: {}", day_one::part_one());
    println!("Day 1, part two: {}", day_one::part_two());
    println!("Day 2, part one: {}", day_two::part_one());
    println!("Day 2, part two: {}", day_two::part_two());
    println!("Day 3, part one: {}", day_three::part_one());
    println!("Day 3, part two: {}", day_three::part_two());
    println!("Day 6, part one: {}", day_six::make_fish(80));
    println!("Day 6, part two: {}", day_six::make_fish(256));
}
