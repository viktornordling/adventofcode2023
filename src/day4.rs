use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;
use regex::Regex;

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input4.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect();  // gather them together into a vector

    let mut sum = 0;
    let mut winners: HashMap<i32, i32> = HashMap::new();
    let mut cards: HashMap<i32, i32> = HashMap::new();
    let mut i = 1;
    for line in &lines {
        let parts: Vec<&str> = line.split(":").collect();
        let board: Vec<&str> = parts[1].split("|").collect();
        let left = &board[0];
        let right = &board[1];
        let re = Regex::new(r"\s+").unwrap();
        let left_nums: Vec<&str> = re.split(left.trim()).collect();
        let right_nums: Vec<&str> = re.split(right.trim()).collect();
        let mut nums_left: HashSet<i32> = HashSet::new();
        let mut nums_right: HashSet<i32> = HashSet::new();
        for num in left_nums {
            nums_left.insert(num.parse().unwrap());
        }
        for num in right_nums {
            nums_right.insert(num.parse().unwrap());
        }
        let correct = nums_left.intersection(&nums_right).count();
        println!("Correct = {}", correct);
        winners.insert(i, correct as i32);
        println!("Setting winners for {} to {}", i, correct);
        cards.insert(i, 1);
        if correct > 0 {
            let points = 1 << (correct - 1);
            println!("Points: {}", points);
            sum += points;
        }
        i += 1;
    }
    println!("Part 1: {}", sum);

    for i in 1..lines.len() {
        println!("Getting winners for {}", i);
        let c = winners[&(i as i32)];
        let copies = cards[&(i as i32)];
        for j in (i + 1)..(c as usize + i + 1) {
            println!("Adding {} copies of card {}", copies, j);
            cards.insert(j as i32, cards[&(j as i32)] + copies);
        }
    }
    let mut sum2 = 0;
    for val in cards.values() {
        sum2 += val;
    }
    println!("Part 2: {}", sum2);
}
