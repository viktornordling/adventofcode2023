use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
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

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input17.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let grid: Vec<Vec<char>> = lines.iter()
        .map(|line| line.chars().collect())
        .collect();

    let min_heat_loss = dijkstra(&grid, 1, 3);
    println!("Part 1: {}", min_heat_loss);

    let min_heat_loss_ultra = dijkstra(&grid, 4, 10);
    println!("Part 2: {}", min_heat_loss_ultra);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct State {
    heat: i32,
    pos: Pos,
    dir: Pos,
    steps: i8,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.heat.hash(state);
        self.pos.hash(state);
        self.dir.hash(state);
        self.steps.hash(state);
    }
}


impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.heat.cmp(&other.heat);
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: &Vec<Vec<char>>, min_steps: i8, max_steps: i8) -> i32 {
    let mut min_heap: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    let up = Pos { x: 0, y: -1 };
    let down = Pos { x: 0, y: 1 };
    let left = Pos { x: -1, y: 0 };
    let right = Pos { x: 1, y: 0 };

    let dirs = &vec![up, down, left, right];

    min_heap.push(Reverse(State { heat: 0, pos: Pos{ x: 0, y: 0}, dir: right, steps: 0}));
    min_heap.push(Reverse(State { heat: 0, pos: Pos{ x: 0, y: 0}, dir: down, steps: 0}));
    let mut seen: HashSet<(Pos, i8, Pos)> = HashSet::new();
    let bottom_right = Pos { x: (grid[0].len() - 1) as i32, y: (grid.len() - 1) as i32 };

    while min_heap.len() > 0 {
        let u = min_heap.pop().unwrap().0;
        if u.pos.eq(&bottom_right) {
            return u.heat;
        }
        if seen.contains(&(u.pos, u.steps, u.dir)) {
            continue;
        }
        seen.insert((u.pos, u.steps, u.dir));
        let last_dir = u.dir;
        let cur_moves_in_one_dir = u.steps;
        let cur_pos = u.pos;
        for dir in dirs.clone() {
            if dir == last_dir && cur_moves_in_one_dir == max_steps {
                continue;
            }
            if dir == up && last_dir == down {
                continue;
            }
            if dir == down && last_dir == up {
                continue;
            }
            if dir == left && last_dir == right {
                continue;
            }
            if dir == right && last_dir == left {
                continue;
            }
            let mut new_pos = Pos { x: cur_pos.x + dir.x, y: cur_pos.y + dir.y};
            if new_pos.x < 0 || new_pos.x >= grid[0].len() as i32 {
                continue;
            }
            if new_pos.y < 0 || new_pos.y >= grid.len() as i32 {
                continue;
            }
            let char = grid[new_pos.y as usize][new_pos.x as usize];
            let mut loss: i32 = char.to_string().parse().ok().unwrap();
            let new_steps = if dir == last_dir {
                cur_moves_in_one_dir + 1
            } else {
                // move at least min_steps in this dir
                for _ in 0..min_steps-1 {
                    new_pos = Pos { x: new_pos.x + dir.x, y: new_pos.y + dir.y};
                    if new_pos.x < 0 || new_pos.x >= grid[0].len() as i32 {
                        break;
                    }
                    if new_pos.y < 0 || new_pos.y >= grid.len() as i32 {
                        break;
                    }
                    let char = grid[new_pos.y as usize][new_pos.x as usize];
                    let new_loss: i32 = char.to_string().parse().ok().unwrap();
                    loss += new_loss;
                }
                min_steps
            };
            let new_state = State { heat: u.heat + loss, pos: Pos{ x: new_pos.x, y: new_pos.y}, dir: dir.clone(), steps: new_steps};
            min_heap.push(Reverse(new_state));
        }
    }
    panic!("Should not get here!");
}

fn _print_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}
