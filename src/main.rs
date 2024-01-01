use std::cmp::{min, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
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

    // let right = Pos { x: 1, y: 0 };
    // let mut seen = HashSet::new();
    // let mut cur_path = Vec::new();
    // let mut cache = HashMap::new();

    let mut seen: HashSet<State> = HashSet::new();
    let s = State { heat: 37, pos: Pos { x: 2, y: 1 }, dir: Pos { x: 1, y: 0 }, steps: 1 };
    seen.insert(s);
    let s2 = State { heat: 37, pos: Pos { x: 2, y: 1 }, dir: Pos { x: 1, y: 0 }, steps: 1 };
    if seen.contains(&s2) {
        println!("Contains!");
    }


    let min_heat_loss = dijkstra(&grid);
    println!("Part 1: {}", min_heat_loss);
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

fn dijkstra(grid: &Vec<Vec<char>>) -> i32 {
    let mut dist: HashMap<Pos, i32> = HashMap::new();
    let mut prev: HashMap<Pos, Pos> = HashMap::new();
    let mut min_heap: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    let up = Pos { x: 0, y: -1 };
    let down = Pos { x: 0, y: 1 };
    let left = Pos { x: -1, y: 0 };
    let right = Pos { x: 1, y: 0 };

    let dirs = &vec![up, down, left, right];

    min_heap.push(Reverse(State { heat: 0, pos: Pos{ x: 0, y: 0}, dir: right, steps: 0}));
    min_heap.push(Reverse(State { heat: 0, pos: Pos{ x: 0, y: 0}, dir: down, steps: 0}));
    let mut seen: HashSet<State> = HashSet::new();

    while min_heap.len() > 0 {
        let u = min_heap.pop().unwrap().0;
        // println!("Popped state: {:?}", u);
        seen.insert(u);
        let last_dir = u.dir;
        let cur_moves_in_one_dir = u.steps;
        let cur_pos = u.pos;
        for dir in dirs.clone() {
            if dir == last_dir && cur_moves_in_one_dir == 3 {
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
            let new_pos = Pos { x: cur_pos.x + dir.x, y: cur_pos.y + dir.y};
            if new_pos.x < 0 || new_pos.x >= grid[0].len() as i32 {
                continue;
            }
            if new_pos.y < 0 || new_pos.y >= grid.len() as i32 {
                continue;
            }
            let new_steps = if dir == last_dir {
                cur_moves_in_one_dir + 1
            } else {
                1
            };
            let char = grid[new_pos.y as usize][new_pos.x as usize];
            let loss: i32 = char.to_string().parse().ok().unwrap();
            let new_state = State { heat: u.heat + loss, pos: Pos{ x: new_pos.x, y: new_pos.y}, dir: dir.clone(), steps: new_steps};
            if !seen.contains(&new_state) {
                println!("seen.len: {}", seen.len());
                min_heap.push(Reverse(new_state));
            } else {
                println!("Already seen state: {:?}", new_state);
            }
            let heat = dist.entry(new_state.pos).or_insert(999999).clone();
            if new_state.heat < heat {
                dist.insert(new_state.pos, new_state.heat);
                prev.insert(new_state.pos, u.pos);
            }
        }
    }
    return dist.get(&Pos{ x: grid[0].len() as i32, y: grid.len() as i32 }).unwrap().clone();
}

fn _dfs(
    cur_pos: Pos,
    last_dir: Pos,
    cur_moves_in_one_dir: i8,
    heat_loss: i32,
    grid: &Vec<Vec<char>>,
    seen: &mut HashSet<Pos>,
    cur_path: &mut Vec<(Pos, Pos)>,
    cache: &mut HashMap<(Pos, Pos, i8), (i32, i32)>
) -> i32 {
    if cache.contains_key(&(cur_pos, last_dir, cur_moves_in_one_dir)) {
        let cached_loss = cache.get(&(cur_pos, last_dir, cur_moves_in_one_dir)).unwrap().clone();
        if cached_loss.0 < heat_loss {
            return cached_loss.1;
        }
    }
    let up = Pos { x: 0, y: -1 };
    let down = Pos { x: 0, y: 1 };
    let left = Pos { x: -1, y: 0 };
    let right = Pos { x: 1, y: 0 };

    let _dir_chars = HashMap::from([
        (up, '^'),
        (down, 'v'),
        (left, '<'),
        (right, '>'),
    ]);

    // println!("Current pos: {:?}, heat_loss: {}, cur_moves_in_one_dir: {}", cur_pos, heat_loss, cur_moves_in_one_dir);
    if cur_pos.x == grid[0].len() as i32 - 1 && grid.len() as i32 - 1 == cur_pos.y {
        // println!("Reached the goal. Path is: {:?}", cur_path);
        // let mut map = grid.clone();
        // for pos in cur_path {
        //     let char = dir_chars.get(&pos.1).unwrap();
        //     map[pos.0.y as usize][pos.0.x as usize] = char.clone();
        // }
        // println!("Reached the goal with heat loss {}", heat_loss);
        // print_grid(&map);
        // println!();
        return heat_loss;
    }

    let dirs = vec![up, down, left, right];

    let mut lowest_heat_loss = 999999;

    for dir in dirs {
        if dir == last_dir && cur_moves_in_one_dir == 3 {
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
        let new_pos = Pos { x: cur_pos.x + dir.x, y: cur_pos.y + dir.y};
        if seen.contains(&new_pos) {
            continue;
        }
        if new_pos.x < 0 || new_pos.x >= grid[0].len() as i32 {
            continue;
        }
        if new_pos.y < 0 || new_pos.y >= grid.len() as i32 {
            continue;
        }
        let new_moves_in_dir = if dir == last_dir {
            cur_moves_in_one_dir + 1
        } else {
            1
        };
        let char = grid[new_pos.y as usize][new_pos.x as usize];
        let loss: i32 = char.to_string().parse().ok().unwrap();
        cur_path.push((new_pos, dir));
        seen.insert(cur_pos);
        // if cur_pos.y == 3 && cur_pos.x == 1 {
        //     println!("Trying to move {:?}", dir);
        // }
        let rest_heat_loss = _dfs(new_pos, dir, new_moves_in_dir, heat_loss + loss, grid, seen, cur_path, cache);
        // if cur_pos.y == 3 && cur_pos.x == 1 {
        //     println!("Result of dfs: {}", rest_heat_loss);
        // }
        seen.remove(&cur_pos);
        cur_path.remove(cur_path.len() - 1);
        lowest_heat_loss = min(lowest_heat_loss, rest_heat_loss);
    }
    cache.insert((cur_pos, last_dir, cur_moves_in_one_dir), (heat_loss, lowest_heat_loss));
    return lowest_heat_loss;
}

fn _print_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}
