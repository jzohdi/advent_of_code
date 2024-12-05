use std::{cmp::Ordering, collections::{HashMap, HashSet}};

pub fn solution1(lines: &[String]) {  
    let mut rules: HashMap<String, HashSet<String>> = HashMap::new();
    let mut switch_mode = false;
    let mut update_lines = vec![];
    for line in lines {
        if is_empty_line(line) {
            switch_mode = true;
            continue;
        }
        if switch_mode {
            update_lines.push(String::from(line));
            continue;
        } 
        let split_line: Vec<&str> = line.split('|').collect();
        let a = String::from(split_line[0]);
        let b = String::from(split_line[1]);
        // println!("split_line: a: {}, b: {}", split_line[0], split_line[1]);
        rules.entry(a).or_insert_with(HashSet::new)
        .insert(b);
    }
    let mut total = 0;
    for line in update_lines {
        let update: Vec<&str> = line.split(",").collect();
        let mut copy_update = update.clone();
        copy_update.sort_by(|a, b| {
            if let Some(set) = rules.get(*a) {
                if set.contains(*b) {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        });
        if copy_update == update {
            println!("line is sorted: {:?}", update);
            total += String::from(*(get_middle_element(&update).unwrap())).parse::<i32>().unwrap();
        }
        // let is_sorted = update
        //     .iter()
        //     .zip(update.iter().sorted())
        //     .all(|(a, b)| a == b);
    }
    // update_lines.sort_by(|a, b| {
    //     if let Some(set) = rules.get(a) {
    //         if set.contains(b) {
    //             return Ordering::Less;
    //         }
    //     }
    //     Ordering::Equal
    // });
    println!("total: {}", total);
}

pub fn solution2(lines: &[String]) {  
    println!("{}", lines.len());
}

fn is_empty_line(line: &String) -> bool {
    let cleaned: String = line.chars().filter(|c| !c.is_whitespace()).collect();
    return cleaned.is_empty();
}
fn get_middle_element<T>(vec: &[T]) -> Option<&T> {
    if vec.is_empty() || vec.len() % 2 == 0 {
        None
    } else {
        Some(&vec[vec.len() / 2])
    }
}