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

fn print_map(start_point: i32, points: &HashMap<i32, Point>) {
    let mut cur = start_point;
    while cur != -1 {
        print_row(cur, points);
        print!("\n");
        cur = points[&cur].down;
    }
}

fn print_row(start: i32, points: &HashMap<i32, Point>) {
    let mut cur = start;
    while cur != -1 {
        let mp = points.get(&cur);
        match mp {
            None => {
                println!("No point found for id {}!", cur);
            }
            Some(p) => {
                if p.has_star {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        cur = points[&cur].right;
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

    println!();
    print_map(0, &points);

    // Grow rows.
    let mut cur_id = 0;
    while points.get(&cur_id).unwrap().down != -1 {
        if !line_has_star(cur_id, &mut points) {
            println!("line from y = {} has no stars, adding another one.", points.get(&cur_id).unwrap().initial_y);
            count = duplicate_line_above(cur_id, &mut points, count);
            println!();
            print_map(0, &points);

        }
        cur_id = points.get(&cur_id).unwrap().down;
    }

    // Grow columns
    cur_id = 0;
    while cur_id != -1 {
        if !col_has_star(cur_id, &mut points) {
            count = duplicate_col_to_the_left(cur_id, &mut points, count);
        }
        cur_id = points.get(&cur_id).unwrap().right;
    }
    // Update the x / y of each point.
    set_final_x_y(& mut points);

    let stars: HashSet<i32> = points.values().filter(|p| p.has_star).map(|p| p.id).collect();

    let mut checked: HashSet<(i32, i32)> = HashSet::new();
    let mut tot_dist = 0;
    for star1 in stars.clone() {
        for star2 in stars.clone() {
            if star1!= star2 && !checked.contains(&(star1, star2)) {
                let s1 = points.get(&star1).unwrap();
                let s2 = points.get(&star2).unwrap();
                let dist = abs(s1.final_x - s2.final_x) + abs(s1.final_y - s2.final_y);
                println!("Dist between star in pos {},{} and {},{} is {}", s1.final_x, s1.final_y, s2.final_x, s2.final_y, dist);
                tot_dist += dist;
                checked.insert((s1.id, s2.id));
                checked.insert((s2.id, s1.id));
            }
        }
    }
    println!("Part 1: {}", tot_dist);
}

fn set_final_x_y(points: &mut HashMap<i32, Point>) {
    let mut y = 0;
    let mut cur_id = 0;
    while cur_id != -1 {
        update_row(cur_id, y, points);
        cur_id = points[&cur_id].down;
        y += 1;
    }
}

fn update_row(start_id: i32, y: i32, points: &mut HashMap<i32, Point>) {
    let mut cur_id = start_id;
    let mut x = 0;
    while cur_id != -1 {
        let mut point = points.get_mut(&cur_id).unwrap();
        point.final_y = y;
        point.final_x = x;
        x += 1;
        cur_id = points[&cur_id].right;
    }
}

fn duplicate_line_above(start_point: i32, points: &mut HashMap<i32, Point>, cur_count: i32) -> i32 {
    let mut cur_id = start_point;
    let mut new_id = cur_count;
    while cur_id != -1 {
        let cur = points[&cur_id];
        new_id += 1;
        let mut new_point =  Point { id: new_id, has_star: false, initial_x: cur.initial_x, initial_y: cur.initial_y - 1, left: -1, right: -1, up: -1, down: -1, final_x: -1, final_y: -1 };
        println!("Created new point with id {}", new_id);
        if cur.up != -1 {
            println!("Getting up: {} from cur: {}", cur.up, cur.id);
            let above = points.get_mut(&cur.up).unwrap();
            above.down = new_id;
            new_point.up = above.id;
        }
        new_point.down = cur.id;
        if cur.down != -1 {
            let below = points.get_mut(&cur.down).unwrap();
            below.up = new_id;
        }
        if cur.left != -1 {
            new_point.left = new_id - 1;
        }
        if cur.right != -1 {
            new_point.right = new_id + 1;
        }
        points.insert(new_id, new_point);
        cur_id = cur.right;
    }
    return new_id;
}

fn duplicate_col_to_the_left(start_point: i32, points: &mut HashMap<i32, Point>, cur_count: i32) -> i32 {
    let mut cur_id = start_point;
    let mut new_id = cur_count;
    while cur_id != -1 {
        let cur = points[&cur_id];
        new_id += 1;
        let mut new_point =  Point { id: new_id, has_star: false, initial_x: cur.initial_x, initial_y: cur.initial_y - 1, left: -1, right: -1, up: -1, down: -1, final_x: -1, final_y: -1 };
        if cur.left != -1 {
            let left = points.get_mut(&cur.left).unwrap();
            left.right = new_id;
            new_point.left = left.id;
        }
        new_point.right = cur.id;
        if cur.right != -1 {
            let right = points.get_mut(&cur.right).unwrap();
            right.left = new_id;
        }
        if cur.up != -1 {
            new_point.up = new_id - 1;
        }
        if cur.down != -1 {
            new_point.down = new_id + 1;
        }
        points.insert(new_id, new_point);
        cur_id = cur.down;
    }
    return new_id;
}

fn line_has_star(start_point: i32, points: &mut HashMap<i32, Point>) -> bool {
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
        println!("Getting point {}", cur_id);
        let cur = points[&cur_id];
        if cur.has_star {
            return true;
        }
        cur_id = cur.down;
    }
    return false;
}