use std::collections::HashSet;
use std::fs;

mod utils;

const FILE_PATH: &str = "./src/day_twelve/input.txt";

type Edges<'a> = utils::Edges<'a>;

pub fn part_one() -> usize {
    let file = fs::read_to_string(FILE_PATH).unwrap();
    let edges = utils::make_edges_map(file.lines().map(|l| l.trim()).collect());
    let visited: HashSet<&str> = vec!["start"].into_iter().collect();
    return part_one_helper("start", visited, &edges);
}

fn part_one_helper(curr: &str, visited: HashSet<&str>, edges: &Edges) -> usize {
    let mut results: usize = 0;

    if curr == "end" {
        return 1;
    };
    if !edges.contains_key(curr) {
        return results;
    };
    for &neighbor in edges.get(curr).unwrap() {
        if !utils::is_small(neighbor) {
            results += part_one_helper(neighbor, visited.clone(), edges);
        } else if !visited.contains(neighbor) {
            let mut visited = visited.clone();
            visited.insert(neighbor);
            results += part_one_helper(neighbor, visited, edges);
        }
    }
    results
}

pub fn part_two() -> usize {
    let file = fs::read_to_string(FILE_PATH).unwrap();
    let edges = utils::make_edges_map(file.lines().map(|l| l.trim()).collect());
    let visited: HashSet<&str> = vec!["start"].into_iter().collect();
    return part_two_helper("start", &visited, false, &edges);
}

fn part_two_helper(curr: &str, visited: &HashSet<&str>, has_doubled: bool, edges: &Edges) -> usize {
    let mut results: usize = 0;

    if curr == "end" {
        return 1;
    };
    if !edges.contains_key(curr) {
        return results;
    };
    for &neighbor in edges.get(curr).unwrap() {
        if neighbor == "start" {
            continue;
        }
        if !utils::is_small(neighbor) {
            results += part_two_helper(neighbor, visited, has_doubled, edges);
        } else if !visited.contains(neighbor) {
            let mut visited = visited.clone();
            visited.insert(neighbor);
            results += part_two_helper(neighbor, &visited, has_doubled, edges);
        } else if !has_doubled {
            results += part_two_helper(neighbor, visited, true, edges);
        }
    }
    results
}
