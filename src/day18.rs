use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::ops::Index;

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

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input18.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut map: HashMap<Pos, char> = HashMap::new();

    let mut cur_pos = Pos { x: 0, y: 0 };

    let up = Pos { x: 0, y: -1 };
    let down = Pos { x: 0, y: 1 };
    let left = Pos { x: -1, y: 0 };
    let right = Pos { x: 1, y: 0 };

    let dir_map: HashMap<String, Pos> = HashMap::from([
        ("D".to_string(), down),
        ("R".to_string(), right),
        ("U".to_string(), up),
        ("L".to_string(), left),
    ]);

    let dir_map2: HashMap<i8, Pos> = HashMap::from([
        (0, right),
        (1, down),
        (2, left),
        (3, up),
    ]);

    let mut points: Vec<Pos> = Vec::new();
    for line in &lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let dir = &parts[0];
        let steps: i32 = parts[1].parse().ok().unwrap();
        let step = dir_map.get(&dir.to_string()).unwrap();
        for _ in 0..steps {
            cur_pos = Pos { x: cur_pos.x + step.x, y: cur_pos.y + step.y };
            map.insert(cur_pos.clone(), '#');
            points.push(cur_pos.clone());
        }
    }
    // print_grid(&map);
    let mut seen = HashSet::new();
    flood_fill(&mut map, Pos { x: 4, y: 1 }, &mut seen);
    print_grid(&map);
    let area = calc_area(&points);
    println!("Part 1: {}", map.len());
    println!("Part 1 mathy: {}", area);

    // let mut new_map: HashMap<Pos, char> = HashMap::new();
    let mut points2: Vec<Pos> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let hex = parts[2];
        let instr = hex.index(2..hex.len() - 2);
        let steps = i32::from_str_radix(instr, 16).unwrap();
        let dir: i32 = hex.index(hex.len() - 2..hex.len() - 1).parse().ok().unwrap();
        let step = dir_map2.get(&(dir as i8)).unwrap();
        for _ in 0..steps {
            cur_pos = Pos { x: cur_pos.x + step.x, y: cur_pos.y + step.y };
            // map.insert(cur_pos.clone(), '#');
            points2.push(cur_pos.clone());
        }
    }

    println!("Part 2: {}", calc_area(&points2));
}

fn calc_area(points: &Vec<Pos>) -> i64 {
    let mut points_with_loop = points.clone();
    points_with_loop.push(points[0]);
    let temp: i64 = points_with_loop.iter()
        .zip(points_with_loop.iter().skip(1))
        .map(|zipped| zipped.0.y as i64 * zipped.1.x as i64 - zipped.1.y as i64 * zipped.0.x as i64).sum();
    let interior_area: f64 = temp as f64 / 2f64;
    let num_interior_points = abs(interior_area) - 0.5 * points.len() as f64 + 1f64;
    return num_interior_points as i64 + points.len() as i64;
}

fn flood_fill(map: &mut HashMap<Pos, char>, cur_pos: Pos, seen: &mut HashSet<Pos>) {
    let up = Pos { x: 0, y: -1 };
    let down = Pos { x: 0, y: 1 };
    let left = Pos { x: -1, y: 0 };
    let right = Pos { x: 1, y: 0 };

    let dirs = &vec![up, down, left, right];

    map.insert(cur_pos, '#');
    seen.insert(cur_pos);

    for dir in dirs {
        let new_pos = Pos { x: cur_pos.x + dir.x, y: cur_pos.y + dir.y };
        if !seen.contains(&new_pos) && !map.contains_key(&new_pos) {
            flood_fill(map, new_pos, seen);
        }
    }
}

fn print_grid(map: &HashMap<Pos, char>) {
    let min_y = map.keys().min_by(|k1, k2| k1.y.cmp(&k2.y)).unwrap().y;
    let max_y = map.keys().max_by(|k1, k2| k1.y.cmp(&k2.y)).unwrap().y;

    let min_x = map.keys().min_by(|k1, k2| k1.x.cmp(&k2.x)).unwrap().x;
    let max_x = map.keys().max_by(|k1, k2| k1.x.cmp(&k2.x)).unwrap().x;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let char = map.get(&Pos { y, x }).unwrap_or_else(|| &'.');
            print!("{}", char);
        }
        println!();
    }
}
