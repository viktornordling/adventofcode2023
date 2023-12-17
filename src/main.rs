mod day8;

use std::collections::HashMap;
use std::fs;
use std::ops::Index;
use num::Integer;

fn find_next(line: &String) -> i32 {
    let nums: Vec<i32> = line.split_whitespace()
        .filter_map(|word| word.parse().ok())
        .collect();
    let mut diffs: Vec<Vec<i32>> = Vec::new();
    let mut cur_diff = nums.clone();
    while !cur_diff.iter().all(|x| *x == 0) {
        let mut new_diffs: Vec<i32> = Vec::new();
        for i in 0..(cur_diff.len() - 1) {
            new_diffs.push(cur_diff[i + 1] - cur_diff[i])
        }
        cur_diff = new_diffs.clone();
        diffs.push(cur_diff.clone());
    }
    return 0
}

fn main() {
    // Part 1.
    let file_path = "/Users/viktor/sources/adventofcode2023/src/input9.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut sum = 0;
    for line in lines {
        let next = find_next(&line);
        sum += next;
    }
    println!("Part 1: {}", sum);
}
