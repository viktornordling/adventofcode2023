extern crate queues;

use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};

use queues::{IsQueue, Queue};

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
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input16.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let grid: Vec<Vec<char>> = lines.iter()
        .map(|line| line.chars().collect())
        .collect();

    let up = Pos { x: 0, y: -1 };
    let down = Pos { x: 0, y: 1 };
    let left = Pos { x: -1, y: 0 };
    let right = Pos { x: 1, y: 0 };

    println!("Part 1: {}", simulate_ray(&grid, Pos { x: 0, y: 0 }, right));

    let mut max_energy = 0;
    for y in 0..grid.len() {
        // Ray from left to right
        max_energy = max(max_energy, simulate_ray(&grid, Pos { x: 0, y: y as i32 }, right));
        // Ray from right to left
        max_energy = max(max_energy, simulate_ray(&grid, Pos { x: (grid[0].len() - 1) as i32, y: y as i32 }, left));
    }

    for x in 0..grid[0].len() {
        // Ray from top to bottom
        max_energy = max(max_energy, simulate_ray(&grid, Pos { x: x as i32, y: 0 }, down));
        // Ray from bottom to top
        max_energy = max(max_energy, simulate_ray(&grid, Pos { x: x as i32, y: (grid.len() - 1) as i32 }, up));
    }

    println!("Part 2: {}", max_energy);
}

fn simulate_ray(grid: &Vec<Vec<char>>, start_pos: Pos, start_dir: Pos) -> usize {
    let up = Pos { x: 0, y: -1 };
    let down = Pos { x: 0, y: 1 };
    let left = Pos { x: -1, y: 0 };
    let right = Pos { x: 1, y: 0 };

    let bounce: HashMap<(char, Pos), Pos> = HashMap::from([
        (('/', up), right),
        (('/', down), left),
        (('/', left), down),
        (('/', right), up),
        (('\\', up), left),
        (('\\', down), right),
        (('\\', left), up),
        (('\\', right), down),
    ]);

    let mut walk: Vec<Vec<char>> = grid.clone();

    let mut seen: HashSet<(Pos, Pos)> = HashSet::new();
    let mut energized: HashSet<Pos> = HashSet::new();
    let mut rays: Queue<(Pos, Pos)> = Queue::new();
    _ = rays.add((start_pos, start_dir));
    while rays.size() > 0 {
        let cur_ray = rays.remove().unwrap();
        // println!("Following ray {:?} in dir {:?}", cur_ray.0, cur_ray.1);
        let mut cur_pos = cur_ray.0;
        let mut cur_dir = cur_ray.1;
        while !seen.contains(&(cur_pos, cur_dir)) {
            // sleep(Duration::from_millis(500));
            seen.insert((cur_pos, cur_dir));
            if cur_pos.x < 0 || cur_pos.x >= grid[0].len() as i32 {
                break;
            }
            if cur_pos.y < 0 || cur_pos.y >= grid.len() as i32 {
                break;
            }
            //walk[cur_pos.y as usize][cur_pos.x as usize] = dirs.get(&cur_dir).unwrap().clone();
            walk[cur_pos.y as usize][cur_pos.x as usize] = '#';
            // print_grid(&walk);
            let cur_char: char = grid[cur_pos.y as usize][cur_pos.x as usize];
            // println!("Current pos: {:?}, current dir: {:?}, current char: {}", cur_pos, cur_dir, cur_char);
            energized.insert(cur_pos);
            if cur_char == '|' && (cur_dir == right || cur_dir == left) {
                cur_dir = down;
                _ = rays.add((cur_pos, up));
            } else if cur_char == '-' && (cur_dir == up || cur_dir == down) {
                cur_dir = right;
                _ = rays.add((cur_pos, left));
            } else if cur_char == '/' || cur_char == '\\' {
                cur_dir = bounce.get(&(cur_char, cur_dir)).unwrap().clone();
            }
            cur_pos = Pos { x: cur_pos.x + cur_dir.x, y: cur_pos.y + cur_dir.y };
        }
    }
    return energized.len();
}

fn _print_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}
