extern crate queues;

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Hash for Pos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    id: i32,
    has_star: bool,
    up: i32,
    down: i32,
    left: i32,
    right: i32,
    initial_x: i32,
    initial_y: i32,
    final_x: i32,
    final_y: i32,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input11.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let grid: Vec<Vec<char>> = lines.iter()
        .map(|line| line.chars().collect())
        .collect();

    for line in &grid {
        for &ch in line {
            print!("{}", ch);
        }
        println!();
    }

    // Create initial points.
    let mut points: HashMap<i32, Point> = HashMap::new();
    let mut count: i32 = 0;
    for y in 0..grid.len() {
        let line = grid.get(y).unwrap();
        for x in 0..line.len() {
            let mut down = -1;
            let mut up = -1;
            let mut left = -1;
            let mut right = -1;
            if x > 0 {
                left = count - 1;
            }
            if x < line.len() - 1 {
                right = count + 1;
            }
            if y > 0 {
                up = count - line.len() as i32;
            }
            if y < grid.len() - 1 {
                down = count + line.len() as i32;
            }
            let has_star = grid[y][x] != '.';
            let point = Point { id: count, has_star, initial_x: x as i32, initial_y: y as i32, left, right, up, down, final_x: -1, final_y: -1 };
            points.insert(count, point);
            count += 1;
        }
    }

    // println!();
    // print_map(0, &points);
    let mut rows_without_stars: HashSet<i32> = HashSet::new();
    let mut cols_without_stars: HashSet<i32> = HashSet::new();

    // Find rows without stars.
    let mut cur_id = 0;
    let mut line_nr = 0;
    while points.get(&cur_id).unwrap().down != -1 {
        if !row_has_star(cur_id, &mut points) {
            rows_without_stars.insert(line_nr);
        }
        cur_id = points.get(&cur_id).unwrap().down;
        line_nr += 1;
    }
    // print_map(0, &points);

    // Grow columns
    cur_id = 0;
    let mut col_nr = 0;
    while cur_id != -1 {
        if !col_has_star(cur_id, &mut points) {
            cols_without_stars.insert(col_nr);
        }
        cur_id = points.get(&cur_id).unwrap().right;
        col_nr += 1;
    }

    let stars: HashSet<i32> = points.values().filter(|p| p.has_star).map(|p| p.id).collect();

    let mut checked: HashSet<(i32, i32)> = HashSet::new();
    let mut tot_dist: i64 = 0;
    let dupes = 1000000;
    for star1 in stars.clone() {
        for star2 in stars.clone() {
            if star1!= star2 && !checked.contains(&(star1, star2)) {
                let s1 = points.get(&star1).unwrap();
                let s2 = points.get(&star2).unwrap();
                let mut y_dist = 0;
                let mut x_dist = 0;
                let y_start = min(s1.initial_y, s2.initial_y);
                let y_end = max(s1.initial_y, s2.initial_y);
                for y in y_start..y_end {
                    if rows_without_stars.contains(&y) {
                        y_dist += dupes;
                    } else {
                        y_dist += 1;
                    }
                }
                let x_start = min(s1.initial_x, s2.initial_x);
                let x_end = max(s1.initial_x, s2.initial_x);
                for x in x_start..x_end {
                    if cols_without_stars.contains(&x) {
                        x_dist += dupes;
                    } else {
                        x_dist += 1;
                    }
                }
                let dist = x_dist + y_dist;
                // println!("Dist between star in pos {},{} and {},{} is {}", s1.final_x, s1.final_y, s2.final_x, s2.final_y, dist);
                tot_dist += dist;
                checked.insert((s1.id, s2.id));
                checked.insert((s2.id, s1.id));
            }
        }
    }
    println!("Part 1: {}", tot_dist);
}

fn row_has_star(start_point: i32, points: &mut HashMap<i32, Point>) -> bool {
    let mut cur_id = start_point;
    while cur_id != -1 {
        let cur = points[&cur_id];
        if cur.has_star {
            return true;
        }
        cur_id = cur.right;
    }
    return false;
}

fn col_has_star(start_point: i32, points: &mut HashMap<i32, Point>) -> bool {
    let mut cur_id = start_point;
    while cur_id != -1 {
        // println!("Getting point {}", cur_id);
        let cur = points[&cur_id];
        if cur.has_star {
            return true;
        }
        cur_id = cur.down;
    }
    return false;
}