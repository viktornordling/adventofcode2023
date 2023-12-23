extern crate queues;

use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};

use num::abs;

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

    let mut grid: Vec<Vec<char>> = lines.iter()
        .map(|line| line.chars().collect())
        .collect();

    for line in &grid {
        for &ch in line {
            print!("{}", ch);
        }
        println!();
    }

    let mut points: HashMap<i32, &Point> = HashMap::new();
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
            points.insert(count, &point);
            count += 1;
        }
    }
    let mut cur = points.get_mut(&0).unwrap();
    while cur.down != -1 {
        if !line_has_star(cur, &mut points) {
            count = duplicate_line_above(&cur, &mut points, count);
        }
        cur = points.get_mut(&cur.down).unwrap();
    }
    cur = points.get_mut(&0).unwrap();
    while cur.down != -1 {
        if !col_has_star(cur, &mut points) {
            count = duplicate_col_to_the_left(&cur, &mut points, count);
        }
        cur = points.get_mut(&cur.right).unwrap();
    }
    // Update the x / y of each point.
    cur = points.get_mut(&0).unwrap();
    let mut y = 0;
    let mut x = 0;
    let mut next_y = cur.id;
    let mut stars = HashSet::new();
    while next_y != -1 {
        let start = points.get_mut(&next_y).unwrap();
        let cur = points.get_mut(&0).unwrap();
        if cur.has_star {
            stars.insert(cur);
        }
        while cur.right != -1 {
            cur.final_x = x;
            cur.final_y = y;
            x += 1;
        }
        y += 1;
        next_y = start.down;
    }
    let mut checked: HashSet<(i32, i32)> = HashSet::new();
    let mut tot_dist = 0;
    for star1 in stars {
        for star2 in stars {
            if star1.id != star2.id && !checked.contains(&(star1.id, star2.id)) {
                tot_dist += abs(star1.final_x - star2.final_x) + abs(star1.final_x - star2.final_x);
            }
        }
    }
    println!("Part 1: {}", tot_dist);
}

fn duplicate_line_above(start_point: &Point, points: &mut HashMap<i32, &Point>, cur_count: i32) -> i32 {
    let mut cur = start_point;
    let mut count = cur_count;
    while cur.right != -1 {
        count += 1;
        let mut new_point =  Point { id: count, has_star: false, initial_x: cur.initial_x, initial_y: cur.initial_y - 1, left: -1, right: -1, up: -1, down: -1, final_x: -1, final_y: -1 };
        if cur.up != -1 {
            let above = points.get_mut(&cur.up).unwrap();
            above.down = count;
            new_point.up = above.id;
        }
        if cur.down != -1 {
            let below = points.get_mut(&cur.down).unwrap();
            below.up = count;
            new_point.down = below.id;
        }
        if cur.left != -1 {
            new_point.left = count - 1;
        }
        if cur.right != -1 {
            new_point.right = count + 1;
        }
        cur = points.get(&cur.right).unwrap();
    }
    return count;
}

fn duplicate_col_to_the_left(start_point: &Point, points: &mut HashMap<i32, Point>, cur_count: i32) -> i32 {
    let mut cur = start_point;
    let mut count = cur_count;
    while cur.down != -1 {
        count += 1;
        let mut new_point =  Point { id: count, has_star: false, initial_x: cur.initial_x, initial_y: cur.initial_y - 1, left: -1, right: -1, up: -1, down: -1, final_x: -1, final_y: -1 };
        if cur.left != -1 {
            let left = points.get_mut(&cur.left).unwrap();
            left.right = count;
            new_point.left = left.id;
        }
        if cur.right != -1 {
            let below = points.get_mut(&cur.down).unwrap();
            below.up = count;
            new_point.down = below.id;
        }
        if cur.up != -1 {
            new_point.up = count - 1;
        }
        if cur.down != -1 {
            new_point.down = count + 1;
        }
        cur = points.get(&cur.down).unwrap();
    }
    return count;
}

fn line_has_star(start_point: &Point, points: &HashMap<i32, &Point>) -> bool {
    let mut cur = start_point;
    while cur.right != -1 {
        if cur.has_star {
            return true;
        }
        cur = points.get(&cur.right).unwrap();
    }
    return false;
}

fn col_has_star(start_point: &Point, points: &HashMap<i32, &Point>) -> bool {
    let mut cur = start_point;
    while cur.down != -1 {
        if cur.has_star {
            return true;
        }
        cur = points.get(&cur.down).unwrap();
    }
    return false;
}