use std::cmp::min;
use std::fs;


fn count_ways(time: i64, dist: i64) -> i32 {
    let mut ways = 0;
    for i in 1..time {
        let speed = i;
        let time_remaining = time - speed;
        let distance = speed * time_remaining;
        println!("Travelled {} in {} millis at speed {}", distance, time_remaining, speed);
        if distance > dist {
            ways += 1;
        }
    }
    return ways;
}

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input6.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let times: Vec<i64> = lines[0]
        .split_whitespace()
        .filter_map(|word| word.parse().ok())
        .collect();

    let distances: Vec<i64> = lines[0]
        .split_whitespace()
        .filter_map(|word| word.parse().ok())
        .collect();

    let mut i = 0;
    let mut result: i64 = 1;
    for time in times {
        let ways = count_ways(time, distances[i]);
        i += 1;
        if ways > 0 {
            result *= ways;
        }
    }

    println!("Part 1: {}", result);
}
