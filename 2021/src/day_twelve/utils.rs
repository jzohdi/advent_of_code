use std::collections::{HashMap, HashSet};

pub type Edges<'a> = HashMap<&'a str, HashSet<&'a str>>;

pub fn is_small(node: &str) -> bool {
    node.to_lowercase() == node
}

pub fn make_edges_map<'a>(lines: Vec<&'a str>) -> Edges<'a> {
    let mut edges: Edges = HashMap::new();

    for line in lines {
        let (s, e) = line.split_once("-").unwrap();
        edges.entry(&s).or_insert(HashSet::new()).insert(e);
        edges.entry(&e).or_insert(HashSet::new()).insert(s);
    }
    edges
}
