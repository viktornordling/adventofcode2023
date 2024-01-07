use std::cmp::min;
use std::fs;

#[derive(Eq, Hash, PartialEq)]
struct SeedRange {
    dest: i64,
    src: i64,
    length: i64,
}

fn parse_range(line: &String) -> SeedRange {
    let nums: Vec<i64> = line
        .split_whitespace()
        .filter_map(|word| word.parse().ok())
        .collect();

    return SeedRange {
        dest: nums[0],
        src: nums[1],
        length: nums[2],
    }
}

fn seed_in_range(seed: i64, range: &SeedRange) -> bool {
    return seed >= range.src && seed < range.src + range.length;
}

fn find_location(seed: i64, maps: &Vec<Vec<SeedRange>>) -> i64 {
    let mut cur = seed;
    for map in maps {
        for range in map.iter() {
            if seed_in_range(cur, range) {
                let offset = range.dest - range.src;
                cur = cur + offset;
                break;
            }
        }
    }
    return cur;
}

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input5.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut lowest: i64 = 4273761942;

    let seeds: Vec<i64> = lines[0]
        .split_whitespace()
        .filter_map(|word| word.parse().ok())
        .collect();

    let mut result: Vec<Vec<SeedRange>> = Vec::new();
    let mut current_vec: Vec<SeedRange> = Vec::new();

    for line in lines.iter().skip(1) {
        if line.ends_with(':') {
            if !current_vec.is_empty() {
                result.push(current_vec);
            }

            current_vec = Vec::new();
        } else if line.is_empty() {
            continue;
        } else {
            current_vec.push(parse_range(line));
        }
    }
    if !current_vec.is_empty() {
        result.push(current_vec);
    }
    for seed in seeds {
        let loc = find_location(seed, &result);
        println!("Final location for seed {} is {}", seed, loc);
        lowest = min(lowest, loc);
    }

    println!("Part 1: {}", lowest);

    // Part 2
    lowest = 9999999999;
    let mut idx = 0;
    // Fuck Rust.
    let seeds2: Vec<i64> = lines[0]
        .split_whitespace()
        .filter_map(|word| word.parse().ok())
        .collect();

    while idx < seeds2.len() {
        let start = seeds2[idx];
        let len = seeds2[idx + 1];
        let mut j = start;
        while j < start + len {
            let seed = j;
            let loc = find_location(seed, &result);
            lowest = min(lowest, loc);
            j += 1;
        }
        idx += 2;
    }
    println!("Part 2: {}", lowest);
}
