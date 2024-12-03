use regex::Regex;

pub fn solution1(lines: &[String]) {    
    let re = Regex::new(r"mul\((?P<num1>\d+),\s*(?P<num2>\d+)\)").unwrap();

    let mut total = 0;
    for line in lines {
        for caps in re.captures_iter(line) {
            let num1: &i32 = &caps["num1"].parse().unwrap();
            let num2: &i32 = &caps["num2"].parse().unwrap();
            total += num1 * num2;
        }
    }
    println!("solution 1 total: {}", total);
    
}

pub fn solution2(lines: &[String]) {    
    // let re = Regex::new(r"do\(\)[^(do\(\)|mul\(\d+,\d+\))]*mul\((?P<num1>\d+),\s*(?P<num2>\d+)\)").unwrap();

    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_mul = Regex::new(r"mul\((?P<num1>\d+),(?P<num2>\d+)\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let text = lines.join("");
    let mut total = 0;
    // let do_matches: Vec<_> = re_do.find_iter(line).collect();
    let mut start_pos = 0;
    while let Some(do_match) = re_do.find(&text[start_pos..]) {
        let do_start = start_pos + do_match.start();
        let do_end = start_pos + do_match.end();
        start_pos = do_end;

        let mut current_pos = do_end;
        let mut found_mul = false;
        while let Some(mul_match) = re_mul.find(&text[current_pos..]) {
            let mul_start = current_pos + mul_match.start();
            let mul_end = current_pos + mul_match.end();

            if let Some(dont_match) = re_dont.find(&text[do_end..mul_start]) {
                break;
            }

            if let Some(caps) = re_mul.captures(&text[mul_start..mul_end]) {
                    let num1: &i32 = &caps["num1"].parse().unwrap();
                    let num2: &i32 = &caps["num2"].parse().unwrap();
                println!(
                    "Found 'mul' at index {} with digits ({}, {}) after 'do()' at index {}.",
                        mul_start, num1, num2, do_start
                );
                found_mul = true;
                total += num1 * num2;
            }

            // Move current position forward
            current_pos = mul_end;
        }

        if !found_mul {
            println!("No 'mul(...)' or 'mult(...)' found after 'do()' at index {}.", do_start);
        }
    }
    println!("solution 2 total: {}", total);
}

// pub fn solution2(lines: &[String]) {    
//     let re = Regex::new(r"do\(\)(?:(?!don't\(\)).)*?mul\((\d+),\d*(\d+)\)").unwrap();

//     let mut total = 0;
//     for line in lines {
//         for caps in re.captures_iter(line) {
//             match caps {
//                 Ok(captures) => {
//                     let num1 = captures.get(1).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
//                     let num2 = captures.get(2).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
                    
//                     total += num1 * num2;
//                 }
//                 Err(e) => println!("Error during matching: {}", e),
//             }
//         }
//     }
//     println!("solution 2 total: {}", total);
// }