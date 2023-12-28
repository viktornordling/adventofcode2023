#![feature(linked_list_remove)]
extern crate queues;

use std::collections::{HashSet, LinkedList};
use std::fs;

fn hash(input: &str) -> i32 {
    let mut current = 0;
    let chars: Vec<char> = input.chars().collect();
    for char in chars {
        let ascii = char as char;
        current += ascii as i32;
        current *= 17;
        current = current % 256;
    }
    return current;
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Lens {
    label: String,
    focal_length: i32,
}

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input15.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let input = &lines[0];
    let parts: Vec<&str> = input.split(",").collect();
    let sum: i32 = parts.iter().map(|str| hash(str)).sum();
    println!("Part 1: {}", sum);

    let mut hash_map: Vec<LinkedList<Lens>> = Vec::with_capacity(256);
    for _ in 0..256 {
        hash_map.push(LinkedList::new());
    }
    let mut lenses: HashSet<String> = HashSet::new();
    for part in parts {
        if part.contains('=') {
            let lens: Vec<&str> = part.split("=").collect();
            let label = &lens[0].to_string();
            lenses.insert(label.to_string());
            let focal_length: i32 = lens[1].to_string().parse().ok().unwrap();
            let hash = hash(&label);
            let list: &mut LinkedList<Lens> = &mut hash_map[hash as usize];
            let mut inserted = false;
            for item in list.iter_mut() {
                if item.label == *label {
                    item.focal_length = focal_length;
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                let l: Lens = Lens { label: label.to_string(), focal_length };
                list.push_back(l);
            }
        } else if part.contains('-') {
            let lens: Vec<&str> = part.split("-").collect();
            let label: String = lens[0].to_string();
            let hash = hash(&label);
            let list: &mut LinkedList<Lens> = &mut hash_map[hash as usize];
            let mut found = false;
            let mut index = 0;
            for item in list.iter_mut() {
                if item.label == label {
                    found = true;
                    break;
                }
                index += 1;
            }
            if found {
                list.remove(index);
            }
        }
    }
    let mut sum = 0;
    for i in 0..256 {
        let list = &hash_map[i];
        let mut idx = 1;
        for lens in list {
            let box_nr = i + 1;
            let slot = idx;
            let focal_length = lens.focal_length;
            let focusing_power = box_nr * slot * focal_length as usize;
            println!("Focusing power for {} is {}", lens.label, focusing_power);
            idx += 1;
            sum += focusing_power;
        }
    }
    println!("Part 2: {}", sum);
}