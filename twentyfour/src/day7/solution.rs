use std::{num::ParseIntError, ops::{Add, Mul}, str::FromStr};

pub fn solution1(lines: &[String]) {  
    let mut total = 0;
    for line in lines {
        let equation = line.parse::<Equation>().unwrap();
        if let Some(target) = equation.is_valid() {
            total += target;
        }
    }

    println!("total: {}", total);
}

fn does_math(target: &Target, numbers: &Vec<Num>, total: Num) -> bool {
    // Base case: If the vector is empty, return
    if numbers.is_empty() {
        return *target == total;
    }

    // Get the first number and the rest of the vector
    let first_num = numbers[0];
    let remaining_numbers = &numbers[1..].to_vec();
    // println!("target: {:?} total: {:?} num: {:?}", target, total, first_num);
    return does_math(target, remaining_numbers, Num(total + first_num)) || 
        does_math(target, remaining_numbers, Num(total * first_num)) ||
        // comment the next line out for part1
        does_math(target, remaining_numbers, total.concat(first_num))
}

pub fn solution2(lines: &[String]) {  
    println!("{}", lines.len());
}

#[derive(Debug)]
struct Target(i64);

impl PartialEq<i64> for Target {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Num> for Target {
    fn eq(&self, other: &Num) -> bool {
        *self == other.0
    }
}


#[derive(Debug, Clone, Copy)]
struct Num(i64);

impl Add<Num> for Num {
    type Output = i64;
    fn add(self, other: Num) -> i64 {
        self.0 + other.0
    }
}

impl Mul<Num> for Num {
    type Output = i64;
    fn mul(self, other: Num) -> i64 {
        self.0 * other.0
    }
}

impl Num {
    fn concat(self, other: Num) -> Num {
        let mut multiplier = 10;
        while other.0 >= multiplier {
            multiplier *= 10;
        }
        Num(self.0 * multiplier + other.0)
    }
}

#[derive(Debug)]
struct Equation {
    target: Target,
    numbers: Vec<Num>,
}

impl Equation {
    fn is_valid(&self) -> Option<i64> {
        // println!("equation: {:?}", self);
        if self.numbers.len() == 0 && self.target == 0 {
            return Some(self.target.0)
        }
        let first_num = self.numbers[0];
        if self.numbers.len() == 1 && self.target == first_num {
            return Some(first_num.0)
        }
        if does_math(&self.target, &self.numbers[1..].to_vec(), first_num) {
            return Some(self.target.0);
        }
        None
    }
}

impl FromStr for Equation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        // Split the string into target and numbers parts
        let mut parts = s.split(':');
        let target_part = parts.next().unwrap();
        let numbers_part = parts.next().unwrap();

        // Parse the target number
        let target_value = target_part.parse::<i64>()?;
        let target = Target(target_value);

        // Parse the list of numbers
        let numbers = numbers_part
            .split_whitespace()
            .map(|num_str| num_str.parse::<i64>().map(Num))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Equation { target, numbers })
    }
}
