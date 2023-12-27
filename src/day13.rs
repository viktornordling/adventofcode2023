extern crate queues;

use std::fs;

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input13.txt";

    let input = fs::read_to_string(file_path).unwrap();
    let cases: Vec<&str> = input.split("\n\n").collect();

    let mut sum: usize = 0;
    for case in &cases {
        let lines: Vec<String> = case
            .lines()
            .map(String::from)
            .collect();
        let grid: Vec<Vec<char>> = lines.iter()
            .map(|line| line.chars().collect())
            .collect();

        let notes = find_reflection(&grid, None);
        sum += notes.unwrap();
    }

    println!("Part 1: {}", sum);

    sum = 0;
    let mut case_id = 0;
    for case in &cases {
        println!("solving case: {}", case_id);
        case_id += 1;
        let lines: Vec<String> = case
            .lines()
            .map(String::from)
            .collect();
        let grid: Vec<Vec<char>> = lines.iter()
            .map(|line| line.chars().collect())
            .collect();
        sum += solve_smudge(grid);
    }
    println!("Part 2: {}", sum);
}

fn solve_smudge(grid: Vec<Vec<char>>) -> usize {
    let unsmudged_solution = find_reflection(&grid, None).unwrap();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut new_grid = grid.clone();
            // print!("Changing x = {}, y = {}", x, y);
            if new_grid[y][x] == '#' {
                // println!(" from # to .");
                new_grid[y][x] = '.';
            } else {
                // println!(" from . to #");
                new_grid[y][x] = '#';
            }
            let result = find_reflection(&new_grid, Option::from(unsmudged_solution));
            if result.is_some() {
                return result.unwrap();
            }
        }
    }
    panic!("No reflection found!")
}

fn find_reflection(grid: &Vec<Vec<char>>, ignore: Option<usize>) -> Option<usize> {
    for y in 0..grid.len() - 1 {
        if lines_equal(&grid[y], &grid[y + 1]) {
            println!("Try folding from y = {}", y);
            let mut low = y as i32 - 1;
            let mut high = (y + 2) as i32;
            let mut reflection = true;
            while low >= 0 && high < grid.len() as i32 {
                if !lines_equal(&grid[low as usize], &grid[high as usize]) {
                    reflection = false;
                    break;
                }
                low -= 1;
                high += 1;
            }
            if reflection {
                // println!("Found reflection from row {}!", y);
                let sol = (y + 1) * 100;
                if ignore.is_some() && ignore.unwrap() == sol {
                    // Ignoring this solution.
                } else {
                    return Option::from(sol);
                }
            }
        }
    }
    let cols = grid[0].len();
    for x in 0..cols - 1 {
        if cols_equal(x, x + 1, &grid) {
            // println!("Try folding from x = {}", x);
            let mut low: i32 = x as i32 - 1;
            let mut high: i32 = (x + 2) as i32;
            let mut reflection = true;
            while low >= 0 && high < cols as i32 {
                if !cols_equal(low as usize, high as usize, &grid) {
                    reflection = false;
                    break;
                }
                low -= 1;
                high += 1;
            }
            if reflection {
                // println!("Found reflection from col {}!", x);
                let sol = x + 1;
                if ignore.is_some() && ignore.unwrap() == sol {
                    // Ignoring this solution.
                } else {
                    return Option::from(sol);
                }
            }
        }
    }
    return None;
}

fn cols_equal(col1: usize, col2: usize, grid: &Vec<Vec<char>>) -> bool {
    for y in 0..grid.len() {
        if grid[y][col1] != grid[y][col2] {
            return false;
        }
    }
    return true;
}

fn lines_equal(line1: &Vec<char>, line2: &Vec<char>) -> bool {
    for x in 0..line1.len() {
        if line1[x] != line2[x] {
            return false;
        }
    }
    return true;
}