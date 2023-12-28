extern crate queues;

use std::fs;

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input14.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let grid: Vec<Vec<char>> = lines.iter()
        .map(|line| line.chars().collect())
        .collect();

    let new_grid: Vec<Vec<char>> = fall_north(&grid);
    // print_grid(&new_grid);
    let score = count_score(&new_grid);
    println!("Part 1: {}", score);

    // Do one cycle.

    let mut cycled = grid;
    for i in 0..1000 {
        cycled = do_cycle(&cycled);
        println!("Score after {} cycles: {}", i + 1, count_score(&cycled));
    }
    let mut i = 714;
    while i <= 1000000000 {
        if i % 100000 == 0 {
            println!("Cycles = {}, score: {}", i, 90928);
        }
        i += 26;
    }
}

fn do_cycle(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let north_grid: Vec<Vec<char>> = fall_north(&grid);
    let west_grid: Vec<Vec<char>> = fall_west(&north_grid);
    let south_grid: Vec<Vec<char>> = fall_south(&west_grid);
    let east_grid: Vec<Vec<char>> = fall_east(&south_grid);
    east_grid
}

fn print_grid(p0: &Vec<Vec<char>>) {
    for y in 0..p0.len() {
        for x in 0..p0[0].len() {
            print!("{}", p0[y][x]);
        }
        println!();
    }
}

fn count_score(grid: &Vec<Vec<char>>) -> usize {
    let mut score = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                score += grid.len() - y;
            }
        }
    }
    return score;
}


fn fall_north(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut fallen = grid.clone();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                // Fall north.
                let mut new_y = y as i32;
                fallen[y][x] = '.';
                while new_y >= 0 && fallen[new_y as usize][x] == '.' {
                    new_y -= 1;
                }
                // println!("Rock in x = {}, y = {} fell from y = {} to y = {}", x, y, y, new_y + 1);
                fallen[(new_y + 1) as usize][x] = 'O';
            }
        }
    }
    return fallen;
}

fn fall_south(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut fallen = grid.clone();
    for y in (0..grid.len()).rev() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                // Fall south.
                let mut new_y = y as i32;
                fallen[y][x] = '.';
                while new_y < grid.len() as i32 && fallen[new_y as usize][x] == '.' {
                    new_y += 1;
                }
                // println!("Rock in x = {}, y = {} fell from y = {} to y = {}", x, y, y, new_y + 1);
                fallen[(new_y - 1) as usize][x] = 'O';
            }
        }
    }
    return fallen;
}

fn fall_east(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut fallen = grid.clone();
    for x in (0..grid[0].len()).rev() {
        for y in 0..grid.len() {
            if grid[y][x] == 'O' {
                // Fall east.
                let mut new_x = x as i32;
                fallen[y][x] = '.';
                while new_x < grid[0].len() as i32 && fallen[y][new_x as usize] == '.' {
                    new_x += 1;
                }
                // println!("Rock in x = {}, y = {} fell from x = {} to x = {}", x, y, y, new_y + 1);
                fallen[y][(new_x - 1) as usize] = 'O';
            }
        }
    }
    return fallen;
}

fn fall_west(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut fallen = grid.clone();
    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if grid[y][x] == 'O' {
                // Fall west.
                let mut new_x = x as i32;
                fallen[y][x] = '.';
                while new_x >= 0 && fallen[y][new_x as usize] == '.' {
                    new_x -= 1;
                }
                // println!("Rock in x = {}, y = {} fell from x = {} to x = {}", x, y, y, new_y + 1);
                fallen[y][(new_x + 1) as usize] = 'O';
            }
        }
    }
    return fallen;
}
