use std::collections::HashMap;
use std::fs;

pub fn solution() -> () {
    println!("Day 16, part one: {}", part_one());
}

fn part_one() -> usize {
    let file = fs::read_to_string("./src/day_16/input.txt").unwrap();
    let binary = file
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| hex_char_to_bin(c))
        .collect::<String>();
    let mut tid = 0;
    let (mut binary, mut version_sum) = parse_slice(&binary, 3);
    let tid = bin_str_to_int(&binary[..3]);
    binary = &binary[3..];

    // if tid == 4 {
    //     let mut t = "";
    //     loop {
    //         t = format!("{}{}", t, binary[1..5]);
    //         let count = binary.chars().next().unwrap();
    //         binary = &binary[5..];
    //         if count == '0' {
    //             break;
    //         }
    //     }
    //     return version_sum
    // } else {
    //     let ltid = binary.chars().next().unwrap();
    //     binary = &binary[1..];
    //     if ltid == '0' {
    //         let packets_length = bin_str_to_int(&binary[..15]);
    //         let packets = &binary[15..];
    //         while packets
    //     }
    // }

    version_sum
}

fn parse_slice(s: &str, length: usize) -> (&str, usize) {
    let num = bin_str_to_int(&s[..length]);
    let rest = &s[length..];
    (rest, num)
}

fn bin_str_to_int(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}

fn hex_char_to_bin(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}
