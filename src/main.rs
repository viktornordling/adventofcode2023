extern crate queues;

use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use queues::*;

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

fn bfs(grid: &mut Vec<Vec<char>>, start: &Pos) {
    let mut dists: Vec<Vec<i32>> = grid.iter()
        .map(|inner_vec| {
            inner_vec.iter()
                .map(|_| 0) // Function is irrelevant. Replaces every element with 0
                .collect() // Collects into Vec<i32>
        })
        .collect();
    let mut queue: Queue<Pos> = Queue::new();
    let mut explored: HashSet<Pos> = HashSet::new();
    explored.insert(start.clone());
    _ = queue.add(start.clone());

    let up = Pos { x: 0, y: -1 };
    let down = Pos { x: 0, y: 1 };
    let left = Pos { x: -1, y: 0 };
    let right = Pos { x: 1, y: 0 };

    let dirs = &vec![up, down, left, right];

    let dir_map = HashMap::from([
        ((&'S', &down), vec!['J', 'L', '|']),
        ((&'S', &up), vec!['F', '7', '|']),
        ((&'S', &left), vec!['-', 'L', 'F']),
        ((&'S', &right), vec!['-', 'J', '7']),
        ((&'|', &down), vec!['J', 'L', '|']),
        ((&'|', &up), vec!['F', '7', '|']),
        ((&'-', &left), vec!['-', 'L', 'F']),
        ((&'-', &right), vec!['-', 'J', '7']),
        ((&'J', &left), vec!['-', 'F', 'L']),
        ((&'J', &up), vec!['|', '7', 'F']),
        ((&'F', &right), vec!['-', 'J', '7']),
        ((&'F', &down), vec!['|', 'J', 'L']),
        ((&'7', &left), vec!['-', 'F', 'L']),
        ((&'7', &down), vec!['|', 'J', 'L']),
        ((&'L', &right), vec!['-', 'J', '7']),
        ((&'L', &up), vec!['|', '7', 'F']),
    ]);

    let mut max_dist = 0;

    while queue.size() > 0 {
        //  6          v := Q.dequeue()
        //  7          if v is the goal then
        //  8              return v
        //  9          for all edges from v to w in G.adjacentEdges(v) do
        // 10              if w is not labeled as explored then
        // 11                  label w as explored
        // 12                  w.parent := v
        // 13                  Q.enqueue(w)
        let el = queue.remove().unwrap();
        let cur_dist = dists[el.y as usize][el.x as usize];
        let cur_char = grid.get(el.y as usize).unwrap().get(el.x as usize).unwrap();
        for dir in dirs {
            let new_pos = Pos { x: el.x + dir.x, y: el.y + dir.y };
            if new_pos.y < 0 || new_pos.x < 0 || new_pos.y >= grid.len() as i32 || new_pos.x >= grid.first().unwrap().len() as i32 || explored.contains(&new_pos) {
                continue
            }
            let nchar = &grid.get(new_pos.y as usize).unwrap().get(new_pos.x as usize).unwrap();
            let valid_neighbors = dir_map.get(&(cur_char, &dir));
            match valid_neighbors {
                None => {}
                Some(neighbors) => {
                    if neighbors.contains(nchar) {
                        explored.insert(new_pos.clone());
                        _ = queue.add(new_pos.clone());
                        dists[new_pos.y as usize][new_pos.x as usize] = cur_dist + 1;
                        max_dist = max(max_dist, cur_dist + 1);
                    }
                }
            }
        }
    }

    for line in &dists {
        for &ch in line {
            if ch > 9 {
                print!("{}", 9);
            } else {
                print!("{}", ch);
            }
        }
        println!();
    }

    println!();

    for y in 0..grid.len() {
        let line: &Vec<char> = &grid[y];
        for x in 0..line.len() {
            if dists[y][x] > 0 {
                print!("{}", grid[y][x]);
            } else {
                print!(".")
            }
        }
        println!();
    }

    println!();

    println!("Part 1: {}", max_dist);

    dists[start.y as usize][start.x as usize] = 1;

    let mut visited: HashSet<Pos> = HashSet::new();
    // let new_start = Pos { x: start.x - 1, y: start.y };
    let new_start = Pos { x: 70, y: 70 };
    grid[start.y as usize][start.x as usize] = 'L';
    dfs(&mut dists, grid, &new_start, &mut visited);

    for line in grid.clone() {
        for ch in line {
            print!("{}", ch);
        }
        println!();
    }
    println!();

    let mut insides = 0;
    for y in 0..grid.len() {
        let line: &Vec<char> = &grid[y];
        for x in 0..line.len() {
            if grid[y as usize][x as usize] == 'I' {
                insides += 1;
            }
        }
        println!();
    }

    // print_grid(&grid);
    println!("Part 2: {}", insides);
}

fn dfs(dists: &mut Vec<Vec<i32>>, grid: &mut Vec<Vec<char>>, cur: &Pos, visited: &mut HashSet<Pos>) {
    visited.insert(cur.clone());
    println!("dfs from x = {}, y = {}, char {}", cur.x, cur.y, grid[cur.y as usize][cur.x as usize]);
    if cur.x == 8 && cur.y == 6 {
        println!("Fuck you in particular.");
    }
    let left_up = (Pos { x: 0, y: 0 }, vec!['-', 'L', 'J']);
    let left_down = (Pos { x: 0, y: 1 }, vec!['-', 'F', '7']);
    let right_up = (Pos { x: 1, y: 0 }, vec!['-', 'L', 'J']);
    let right_down = (Pos { x: 1, y: 1 }, vec!['-', 'F', '7']);

    let up_left = (Pos { x: 0, y: 0 }, vec!['|', '7', 'J']);
    let up_right = (Pos { x: 1, y: 0 }, vec!['|', 'F', 'L']);
    let down_left = (Pos { x: 0, y: 1 }, vec!['|', '7', 'J']);
    let down_right = (Pos { x: 1, y: 1 }, vec!['|', 'F', 'L']);

    let up = Pos { x: 0, y: -1 };
    let down = Pos { x: 0, y: 1 };
    let left = Pos { x: -1, y: 0 };
    let right = Pos { x: 1, y: 0 };

    let up = (up_left, up_right, up);
    let down = (down_left, down_right, down);
    let left = (left_up, left_down, left);
    let right = (right_up, right_down, right);
    let dirs = vec![up, down, left, right];

    for dir in dirs {
        let (first, second, step) = dir;
        let new_pos1 = Pos { x: cur.x + first.0.x, y: cur.y + first.0.y };
        let new_pos2 = Pos { x: cur.x + second.0.x, y: cur.y + second.0.y };
        if new_pos1.y < 0 || new_pos1.x < 0 || new_pos1.y >= grid.len() as i32 || new_pos1.x >= grid.first().unwrap().len() as i32 {
            // We hit the border
            continue
        }
        if new_pos2.y < 0 || new_pos2.x < 0 || new_pos2.y >= grid.len() as i32 || new_pos2.x >= grid.first().unwrap().len() as i32 {
            // We hit the border
            continue
        }
        // If new_pos1 or new_pos2 is not a wall, greedily mark it as "inside".
        if dists[new_pos1.y as usize][new_pos1.x as usize] == 0 {
            println!("Marking {},{} as I", new_pos1.x, new_pos1.y);
            grid[new_pos1.y as usize][new_pos1.x as usize] = 'I';
        }
        if dists[new_pos2.y as usize][new_pos2.x as usize] == 0 {
            println!("Marking {},{} as I", new_pos2.x, new_pos2.y);
            grid[new_pos2.y as usize][new_pos2.x as usize] = 'I';
        }
        // If we can move in this direction, dfs.
        let allowed1 = first.1;
        let allowed2 = second.1;
        let char1 = grid[new_pos1.y as usize][new_pos1.x as usize];
        let char2 = grid[new_pos2.y as usize][new_pos2.x as usize];
        let a = allowed1.contains(&char1) || dists[new_pos1.y as usize][new_pos1.x as usize] == 0;
        let b = allowed2.contains(&char2) || dists[new_pos2.y as usize][new_pos2.x as usize] == 0;
        let next_step = Pos { x: cur.x + step.x, y: cur.y + step.y };
        let c = !visited.contains(&next_step);
        if a && b && c {
            dfs(dists, grid, &next_step, visited);
        }
    }
}

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input10.txt";

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

    let mut s = Pos { x: 0, y: 0 };
    for y in 0..grid.len() {
        let line = grid.get(y).unwrap();
        for x in 0..line.len() {
            let c = line.get(x).unwrap();
            if c == &'S' {
                s = Pos { x: x as i32, y: y as i32 };
            }
        }
    }
    bfs(&mut grid, &s);
}