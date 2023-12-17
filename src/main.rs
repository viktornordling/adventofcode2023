use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::ops::Index;
use std::rc::Rc;

mod day7;

#[derive(Eq, PartialEq, Debug)]
struct Node {
    name: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: &String) -> Self {
        Node {
            name: name.to_owned(),
            left: None,
            right: None,
        }
    }
}

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input8.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut node_map: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();

    for line in lines.iter().skip(2) {
        let parts: Vec<&str> = line.split("=").collect();
        let node_name = parts[0].trim();
        let lr: Vec<&str> = parts[1].split(", ").collect();
        let left = lr[0].index(2..);
        let right = lr[1].index(..lr[1].len() - 1);

        let current_node= node_map.entry(node_name.to_string())
            .or_insert(Rc::new(RefCell::new(Node::new(&node_name.to_string())))).clone();

        let left_node= node_map.entry(left.to_string())
            .or_insert(Rc::new(RefCell::new(Node::new(&left.to_string())))).clone();

        let right_node= node_map.entry(right.to_string())
            .or_insert(Rc::new(RefCell::new(Node::new(&right.to_string())))).clone();


        current_node.borrow_mut().left = Some(left_node.clone());
        current_node.borrow_mut().right = Some(right_node.clone());
    }

    // let dirs = &lines[0];

    // let left = &a.borrow().left.clone().unwrap();
    // let right = &a.borrow().right.clone().unwrap();
    // println!("Left: {}", left.borrow().name);
    // println!("Right: {}", right.borrow().name);

    let mut idx = 0;
    let mut a = node_map.get("AAA").unwrap().borrow_mut();
    let dirs: Vec<char> = lines[0].chars().collect();
    while a.name != "ZZZ" {
        let dir = dirs.get(idx);
        if dir.unwrap() == &'L' {
            let aa = a.left.as_ref().unwrap().clone();
            a = aa.borrow_mut();
        } else {
            a = a.right.as_ref().unwrap().clone().borrow_mut();
        }
        idx += 1;
        if idx >= dirs.len() {
            idx = 0;
        }
    }

    let sum = 0;
    println!("Part 1: {}", sum);

}
