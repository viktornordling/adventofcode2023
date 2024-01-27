use std::collections::HashMap;
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
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input20.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let grid: Vec<Vec<char>> = lines.iter()
        .map(|line| line.chars().collect())
        .collect();
    println!("Grid width = {}, grid height = {}", grid.len(), grid[0].len());
    let mut big_grid: HashMap<Pos, char> = HashMap::new();
    for cell_y in (-5) as i32..5 {
        for cell_x in (-5) as i32..5 {
            for y in 0..131 {
                for x in 0..131 {
                    let some_x = cell_x * 131 + x;
                    let some_y = cell_y * 131 + y;
                    let the_char = grid[y as usize][x as usize];
                    if the_char == 'S' && !(cell_x == 0 && cell_y == 0) {
                        big_grid.insert(Pos { x: some_x as i32, y: some_y as i32 }, '.');
                    } else {
                        big_grid.insert(Pos { x: some_x as i32, y: some_y as i32 }, the_char);
                    }
                }
            }
        }
    }
    println!("BIG GRID!");
    print_big_grid(&big_grid, -131, 262);
    println!("END BIG GRID!");

    println!("ORG GRID!");
    print_grid(&grid);
    println!("END ORG GRID!");

    let mut result = grid.clone();
    let mut big_result = big_grid.clone();

    for i in 0..589 {
        big_result = take_big_step(&big_result);
        // sleep(Duration::from_millis(400));
        // print_big_grid(&big_result, -131, 262);
        println!("{}", i);
    }
    print_big_grid(&big_result, -524, 655);
    // for i in 0..10 {
    //     result = take_step(&result);
    //     print_grid(&result);
    // }

    let mut count = 0;
    for k in big_result {
        if k.1 == 'O' {
            count += 1;
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

fn take_big_step(big_grid: &HashMap<Pos, char>) -> HashMap<Pos, char> {
    let mut result = big_grid.clone();
    let mut start_positions: Vec<Pos> = Vec::new();
    // print_grid(&grid);

    let up = Pos { x: 0, y: -1 };
    let down = Pos { x: 0, y: 1 };
    let left = Pos { x: -1, y: 0 };
    let right = Pos { x: 1, y: 0 };

    let dirs = vec![up, down, left, right];

    for k in big_grid {
        if k.1 == &'S' || k.1 == &'O' {
            result.insert(k.0.clone(), '.');
            start_positions.push(k.0.clone());
        }
    }
    // for y in 0..result.len() {
    //     for x in 0..result[0].len() {
    //         if result[y][x] == 'S' || result[y][x] == 'O' {
    //             result[y][x] = '.';
    //             start_positions.push(Pos {y: y as i32, x: x as i32});
    //         }
    //     }
    // }
    println!("num start positions: {}", start_positions.len());
    for pos in start_positions {
        for dir in &dirs {
            let new_pos = Pos { x: pos.x + dir.x, y: pos.y + dir.y };
            let c = big_grid.get(&new_pos).unwrap();
            // println!("c = {}", c);
            if *c != '#' {
                result.insert(new_pos, 'O');
            }
        }
    }

    return result.clone();
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}

fn print_big_grid(big_grid: &HashMap<Pos, char>, start: i32, end: i32) {
    for y in start..end {
        for x in start..end {
            let c = big_grid.get(&Pos { x, y }).unwrap();
            print!("{}", c);
        }
        println!();
    }
}

