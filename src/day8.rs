
use std::collections::HashMap;
use std::fs;
use std::ops::Index;
use num::Integer;

#[derive(Eq, PartialEq, Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn new(name: &String, left: &String, right: &String) -> Self {
        Node {
            name: name.to_owned(),
            left: left.to_owned(),
            right: right.to_owned(),
        }
    }
}

fn count_steps(start_node: &String, dirs: &Vec<char>, node_map: &HashMap<String, Node>) -> i32 {
    let mut idx = 0;
    let mut a = node_map.get(start_node).unwrap();
    let mut count = 0;
    while !a.name.ends_with('Z') {
        let dir = dirs.get(idx);
        if dir.unwrap() == &'L' {
            a = node_map.get(&a.left).unwrap();
        } else {
            a = node_map.get(&a.right).unwrap();
        }
        idx += 1;
        count += 1;
        if idx >= dirs.len() {
            idx = 0;
        }
    }
    return count;
}

fn main() {
    // Part 1.
    let file_path = "/Users/viktor/sources/adventofcode2023/src/input8.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut node_map: HashMap<String, Node> = HashMap::new();

    for line in lines.iter().skip(2) {
        let parts: Vec<&str> = line.split("=").collect();
        let node_name = parts[0].trim();
        let lr: Vec<&str> = parts[1].split(", ").collect();
        let left = lr[0].index(2..);
        let right = lr[1].index(..lr[1].len() - 1);

        let node = Node::new(&node_name.to_string(), &left.to_string(), &right.to_string());
        node_map.insert(node_name.to_string(), node);
    }

    let dirs: Vec<char> = lines[0].chars().collect();

    let steps = count_steps(&"AAA".to_string(), &dirs, &node_map) as i64;
    println!("Part 1: {}", steps);

    let start_nodes: Vec<String> = node_map.keys().filter(|key| key.ends_with('A')).cloned().collect();
    let cycles: Vec<i64> = start_nodes.iter().map(|start_node| count_steps(start_node, &dirs, &node_map) as i64).collect();
    let mut lcm = 1i64;
    for cycle in cycles {
        lcm = lcm.lcm(&cycle);
    }
    println!("Part 2: {}", lcm);
}
