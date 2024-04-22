use std::collections::HashMap;
use std::fs;
use std::ops::Index;

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/intput.txt";
    println!("In file {}", file_path);

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut total_sum = 0;
    for line in lines.iter() {
        let mut nums = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                nums.push(c);
            }
        }
        let sum: i32 = format!("{}{}", nums[0], nums[nums.len() - 1]).parse().unwrap();
        total_sum += sum;
    }
    println!("Part 1: {}", total_sum);

    // Part 2.
    total_sum = 0;
    let num_map = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    for line in lines.iter() {
        let mut nums = Vec::new();
        let mut index = 0;
        for c in line.chars() {
            if c.is_numeric() {
                nums.push(c);
            } else {
                // two1nine 2 + 9 = '2' + '9' = "29"
                let substring = line.index(index..);
                for num in num_map.keys() {
                    if substring.starts_with(num) {
                        nums.push(num_map[num]);
                    }
                }
            }
            index += 1;
        }
        let sum: i32 = format!("{}{}", nums[0], nums[nums.len() - 1]).parse().unwrap();
        total_sum += sum;
    }
    println!("Part 2: {}", total_sum);
}
