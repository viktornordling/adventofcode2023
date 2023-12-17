use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Index;

#[derive(Eq, Hash, PartialEq)]
struct Pos {
    x: i32,
    y: i32
}

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input3.txt";
    println!("In file {}", file_path);

    let lines: Vec<Vec<char>> = fs::read_to_string(file_path)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .map(|s| s.chars().collect())
        .collect();  // gather them together into a vector

    let mut symbols: HashMap<Pos, char> = HashMap::new();
    let rows: i32 = lines.len() as i32;
    let cols: i32 = lines[0].len() as i32;

    for i in 0..rows {
        let line = &lines[i as usize];
        for j in 0..cols {
            let c: char = line[j as usize];
            if c != '.' && !c.is_numeric() {
                symbols.insert(Pos { x: j, y: i }, c);
            }
        }
    }
    let dirs = vec![Pos {x: -1, y: -1}, Pos {x: 0, y: -1}, Pos {x: 1, y: -1},
                    Pos {x: -1, y: 0}, Pos {x: 1, y: 0},
                    Pos {x: -1, y: 1}, Pos {x: 0, y: 1}, Pos {x: 1, y: 1}, ];
    let mut sum = 0;
    let mut nums: HashSet<Pos> = HashSet::new();
    let mut star_neighbors: HashMap<Pos, Vec<i32>> = HashMap::new();
    for pos in symbols.keys() {
        for dir in &dirs {
            // Find number at pos pos.x + dir.x, pos.y + dir.y.
            let new_pos = Pos { x: pos.x + dir.x, y: pos.y + dir.y};
            if new_pos.x < 0 || new_pos.x >= cols || new_pos.y < 0 || new_pos.y >= rows {
                continue;
            }
            let line = &lines[new_pos.y as usize];
            let c: char = line[new_pos.x as usize];
            if c.is_numeric() {
                let init = new_pos.x;
                let mut start: i32 = init;
                let mut end: usize = init as usize;
                // Read the full number.
                while start >= 0 && line[start as usize].is_numeric() {
                    // start = max(0, start - 1);
                    start -= 1;
                }
                while end < line.len() && line[end].is_numeric() {
                    end += 1;
                }
                let num_string: &str = &String::from(line
                    .iter()
                    .skip((start + 1) as usize)
                    .take((end as i32 - start) as usize - 1)
                    .collect::<String>())
                    .into_boxed_str();
                let num: i32 = num_string.parse().unwrap();
                let num_pos = Pos{x: start, y: new_pos.y};
                if !nums.contains(&num_pos) {
                    nums.insert(num_pos);
                    sum += num;
                    if symbols[pos] == '*' {
                        if let Some(vec) = star_neighbors.get_mut(&pos) {
                            vec.push(num);
                        } else {
                            star_neighbors[pos] = vec![num];
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", sum);

    let mut sum_2 = 0;
    for key in star_neighbors.keys() {
        let vals = &star_neighbors[key];
        if vals.len() == 2 {
            sum_2 += vals[0] * vals[1];
        }
    }
    println!("Part 2: {}", sum_2);
}
