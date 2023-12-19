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

fn bfs(grid: &Vec<Vec<char>>, start: &Pos) {
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
            let nchar = grid.get(new_pos.y as usize).unwrap().get(new_pos.x as usize).unwrap();
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

    dists[start.y as usize][start.x as usize] = 1;

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
    println!("Part 1: {}", max_dist);

    // let mut dots: HashSet<Pos> = HashSet::new();
    // for y in 0..grid.len() {
    //     let line = grid.get(y).unwrap();
    //     for x in 0..line.len() {
    //         let c = line.get(x).unwrap();
    //         if c == &'.' {
    //             dots.add(Pos { x: x as i32, y: y as i32 });
    //         }
    //     }
    // }
    // let mut seen_dots: HashSet<Pos> = HashSet::new();
    // let mut inside_dots = 0;
    // for dot in dots {
    //     if seen_dots.contains(&dot) {
    //         continue;
    //     }
    //     let mut sub_seen: HashSet<Pos> = HashSet::new();
    //     let (visited, inside) = dfs(&dists, &grid, &dot, &mut sub_seen, false);
    //     let c = match inside {
    //         true => 'I',
    //         false => 'O'
    //     };
    //     for vdot in visited {
    //         seen_dots.insert(vdot);
    //         grid[vdot.y][vdot.x] = c;
    //     }
    //     if inside {
    //         inside_dots += visited.len();
    //     }
    // }
    // print_grid(&grid);
    // println!("Part 2: {}", inside_dots);
}

// fn dfs(dists: &Vec<Vec<i32>>, grid: &Vec<Vec<char>>, cur: &Pos, seen: &mut HashSet<Pos>, inside: Option<bool>) -> (HashSet<Pos>, bool) {
//     let up = Pos { x: 0, y: -1 };
//     let down = Pos { x: 0, y: 1 };
//     let left = Pos { x: -1, y: 0 };
//     let right = Pos { x: 1, y: 0 };
//     let dirs = vec![up, down, left, right];
//     let mut final_inside = inside;
//
//     for dir in dirs {
//         let new_pos = Pos { x: cur.x + dir.x, y: cur.y + dir.y };
//         if new_pos.y < 0 || new_pos.x < 0 || new_pos.y >= grid.len() as i32 || new_pos.x >= grid.first().unwrap().len() as i32 {
//             // We hit the border, so we can't be inside.
//             final_inside = Some(false);
//             continue
//         } else if grid[new_pos.y][new_pos.x] == '.' {
//             // DFS into this position.
//         } else if dists[new_pos.y][new_pos.x] > 0 {
//             // We hit pipes, we could still be inside
//         } else {
//             // We hit some other stuff, we can't be inside.
//         }
//     }
//     return (seen.clone(), inside);
// }

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input10.txt";

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
    bfs(&grid, &s);
}