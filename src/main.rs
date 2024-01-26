use std::fs;
use std::hash::{Hash, Hasher};
use std::thread::sleep;
use std::time::Duration;

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
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input20.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let grid: Vec<Vec<char>> = lines.iter()
        .map(|line| line.chars().collect())
        .collect();
    let mut result = grid.clone();

    for i in 0..64 {
        result = take_step(&result);
        sleep(Duration::from_millis(400));
        print_grid(&result);
        println!();
    }
    // for i in 0..10 {
    //     result = take_step(&result);
    //     print_grid(&result);
    // }

    let mut count = 0;
    for y in 0..result.len() {
        for x in 0..result[0].len() {
            if result[y][x] == 'O' {
                count += 1;
            }
        }
    }
    println!("Part 1: {}", count);
}

fn take_step(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = grid.clone();
    let mut start_positions: Vec<Pos> = Vec::new();
    // print_grid(&grid);

    let up = Pos { x: 0, y: -1 };
    let down = Pos { x: 0, y: 1 };
    let left = Pos { x: -1, y: 0 };
    let right = Pos { x: 1, y: 0 };

    let dirs = vec![up, down, left, right];

    for y in 0..result.len() {
        for x in 0..result[0].len() {
            if result[y][x] == 'S' || result[y][x] == 'O' {
                result[y][x] = '.';
                start_positions.push(Pos {y: y as i32, x: x as i32});
            }
        }
    }
    for pos in start_positions {
        for dir in &dirs {
            let new_pos = Pos { x: pos.x + dir.x, y: pos.y + dir.y };
            if new_pos.y >= 0 && new_pos.x >= 0 && new_pos.y < grid.len() as i32 && new_pos.x < grid.first().unwrap().len() as i32 && grid[new_pos.y as usize][new_pos.x as usize] != '#' {
                result[new_pos.y as usize][new_pos.x as usize] = 'O';
            }
        }
    }

    return result;
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}
