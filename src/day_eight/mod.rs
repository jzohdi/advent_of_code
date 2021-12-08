use std::collections::{HashMap, HashSet};
use std::fs;
const FILE_PATH: &str = "./src/day_eight/input.txt";

pub fn solution() -> usize {
    let file = fs::read_to_string(FILE_PATH).unwrap();
    file.lines()
        .map(|line| line.split_once(" | ").unwrap().1)
        .flat_map(|output| output.split(" "))
        .filter(|word| word.len() == 7 || word.len() == 2 || word.len() == 3 || word.len() == 4)
        .count()
}

pub fn solution_part_two() -> usize {
    let file = fs::read_to_string(FILE_PATH).unwrap();
    file.lines().map(|line| deduce_line_output(line)).sum()
}

// unique nums = 1, 4, 7, 8
// get these first
// top letter = 7 take away 1
// 2 segments missing = 2, 3, 5
// |----> if 2 segments and shared with 1 = 3
// |----> if 2 segments and shared with (4 take away 1) = 5
// |----> last one left = 2

fn deduce_line_output(line: &str) -> usize {
    let input_output_tup = line.split_once(" | ").unwrap();
    let input_words: Vec<&str> = input_output_tup.0.split(" ").collect();
    let (mut str_mapping, int_mapping): (HashMap<&str, usize>, HashMap<usize, &str>) =
        deduce_easy_nums(input_words.clone());
    let output_words: Vec<&str> = input_output_tup.1.split(" ").collect();

    let five_segmnets: Vec<&str> = input_words
        .clone()
        .into_iter()
        .filter(|word| word.len() == 5)
        .collect();

    let (three, five, two) = deduce_five_segments(five_segmnets, int_mapping.clone());
    str_mapping.insert(three, 3);
    str_mapping.insert(five, 5);
    str_mapping.insert(two, 2);

    let six_segments: Vec<&str> = input_words
        .clone()
        .into_iter()
        .filter(|word| word.len() == 6)
        .collect();

    let (nine, zero, six) = deduce_six_segments(six_segments, int_mapping);
    str_mapping.insert(nine, 9);
    str_mapping.insert(zero, 0);
    str_mapping.insert(six, 6);

    let strings = output_words
        .into_iter()
        .map(|word| {
            let mut answer = 0;
            for (string, num) in str_mapping.clone() {
                if string.chars().all(|s| word.contains(s))
                    && word.chars().all(|w| string.contains(w))
                {
                    answer = num;
                }
            }
            answer.to_string()
        })
        .collect::<Vec<String>>();
    return strings.join("").parse::<usize>().unwrap();
}

// 1 segment missing = 0, 6, 9
// |----> if 1 segment missing and shared with 4 = 9
// |----> remove 9, and if 1 seg and shared with 1 = 0
// |----> last one left = 6
fn deduce_six_segments<'a>(
    words: Vec<&'a str>,
    int_mapping: HashMap<usize, &str>,
) -> (&'a str, &'a str, &'a str) {
    let four = (*int_mapping.get(&4).unwrap())
        .chars()
        .collect::<HashSet<char>>();
    let nine = words
        .clone()
        .into_iter()
        .filter(|&word| word.chars().filter(|w| four.contains(w)).count() == 4)
        .collect::<Vec<&str>>();
    if nine.len() != 1 {
        panic!("could not find nine");
    }
    let nine = nine[0];

    let one = (*int_mapping.get(&1).unwrap())
        .chars()
        .collect::<HashSet<char>>();
    let zero = words
        .clone()
        .into_iter()
        .filter(|&word| !word.eq(nine) && word.chars().filter(|w| one.contains(w)).count() == 2)
        .collect::<Vec<&str>>();
    if zero.len() != 1 {
        panic!("could not find zero");
    }
    let zero = zero[0];

    let six = words
        .into_iter()
        .filter(|word| !word.eq(&nine) && !word.eq(&zero))
        .collect::<Vec<&str>>();
    if six.len() != 1 {
        panic!("could not find six");
    }

    let six = six[0];

    // println!("nine: {:?}\n", nine);
    (nine, zero, six)
}

fn deduce_five_segments<'a>(
    words: Vec<&'a str>,
    int_mapping: HashMap<usize, &str>,
) -> (&'a str, &'a str, &'a str) {
    let one = (*int_mapping.get(&1).unwrap())
        .chars()
        .collect::<HashSet<char>>();
    // println!("5 segments: {:?}", words);
    let three = words
        .clone()
        .into_iter()
        .filter(|&word| word.chars().filter(|w| one.contains(w)).count() == 2)
        .collect::<Vec<&str>>();

    let four_take_away_one = (*int_mapping.get(&4).unwrap())
        .chars()
        .filter(|f| !one.contains(f))
        .collect::<HashSet<char>>();
    let five = words
        .clone()
        .into_iter()
        .filter(|&word| {
            word.chars()
                .filter(|w| four_take_away_one.contains(w))
                .count()
                == 2
        })
        .collect::<Vec<&str>>();
    if three.len() != 1 || five.len() != 1 {
        panic!("three len and five len not equal 1");
    }
    let three = three[0];
    let five = five[0];
    let two = words
        .into_iter()
        .filter(|word| !word.eq(&three) && !word.eq(&five))
        .collect::<Vec<&str>>();
    if two.len() != 1 {
        panic!("could not get two");
    }
    (three, five, two[0])
}

fn deduce_easy_nums<'a>(
    input_words: Vec<&'a str>,
) -> (HashMap<&'a str, usize>, HashMap<usize, &'a str>) {
    let mut str_mapping: HashMap<&str, usize> = HashMap::new();
    let mut int_mapping: HashMap<usize, &str> = HashMap::new();
    let one: &str = input_words
        .iter()
        .filter(|word| word.len() == 2)
        .map(|word| *word)
        .collect::<Vec<&str>>()[0];
    str_mapping.insert(one, 1);
    int_mapping.insert(1, one);
    let four: &str = input_words
        .iter()
        .filter(|word| word.len() == 4)
        .map(|word| *word)
        .collect::<Vec<&str>>()[0];
    str_mapping.insert(four, 4);
    int_mapping.insert(4, four);
    let seven: &str = input_words
        .iter()
        .filter(|word| word.len() == 3)
        .map(|word| *word)
        .collect::<Vec<&str>>()[0];
    str_mapping.insert(seven, 7);
    int_mapping.insert(7, seven);
    let eight: &str = input_words
        .iter()
        .filter(|word| word.len() == 7)
        .map(|word| *word)
        .collect::<Vec<&str>>()[0];
    str_mapping.insert(eight, 8);
    int_mapping.insert(8, eight);
    (str_mapping, int_mapping)
}
