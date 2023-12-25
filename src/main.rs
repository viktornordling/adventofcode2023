extern crate queues;

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input12.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

}