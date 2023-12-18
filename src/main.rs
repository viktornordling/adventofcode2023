use std::fs;

fn find_next(line: &String) -> i32 {
    let nums: Vec<i32> = line.split_whitespace()
        .filter_map(|word| word.parse().ok())
        .collect();
    let mut diffs: Vec<Vec<i32>> = Vec::new();
    let mut cur_diff = nums.clone();
    // let aa = cur_diff.iter().all(|x| *x == 0);
    while !all_zero(&cur_diff) {
        let mut new_diffs: Vec<i32> = Vec::new();
        for i in 0..(cur_diff.len() - 1) {
            new_diffs.push(cur_diff[i + 1] - cur_diff[i])
        }
        cur_diff = new_diffs.clone();
        diffs.push(cur_diff.clone());
    }
    let mut num = 0;
    for i in 0..diffs.len() - 1 {
        let cc = diffs.get(diffs.len() - i - 2).unwrap();
        println!("cc.last = {}, num = {}", cc.last().unwrap(), num);
        num = cc.last().unwrap() + num;
        println!("new num = {}", num);
    }
    println!("num = {}", num + nums.last().unwrap());
    return num + nums.last().unwrap();
}

fn find_prev(line: &String) -> i32 {
    let nums: Vec<i32> = line.split_whitespace()
        .filter_map(|word| word.parse().ok())
        .collect();
    let mut diffs: Vec<Vec<i32>> = Vec::new();
    let mut cur_diff = nums.clone();
    // let aa = cur_diff.iter().all(|x| *x == 0);
    while !all_zero(&cur_diff) {
        let mut new_diffs: Vec<i32> = Vec::new();
        for i in 0..(cur_diff.len() - 1) {
            new_diffs.push(cur_diff[i + 1] - cur_diff[i])
        }
        cur_diff = new_diffs.clone();
        diffs.push(cur_diff.clone());
    }
    let mut num = 0;
    for i in 0..diffs.len() - 1 {
        let cc = diffs.get(diffs.len() - i - 2).unwrap();
        println!("cc.last = {}, num = {}", cc.last().unwrap(), num);
        num = cc.first().unwrap() - num;
        println!("new num = {}", num);
    }
    println!("num = {}", nums.first().unwrap() - num);
    return nums.first().unwrap() - num;
}

fn all_zero(cur_diff: &Vec<i32>) -> bool {
    let all_zero = cur_diff.iter().all(|x| *x == 0);
    println!("All zero? {}", all_zero);
    return all_zero;
}

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input9.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut sum = 0;
    for line in lines.clone() {
        let next = find_next(&line);
        sum += next;
    }
    println!("Part 1: {}", sum);

    let mut sum2 = 0;
    for line in lines {
        let prev = find_prev(&line);
        sum2 += prev;
    }
    println!("Part 2: {}", sum2);
}
