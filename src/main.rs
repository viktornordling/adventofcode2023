extern crate queues;

use std::fs;
use std::hash::Hash;

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input12.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut sum = 0;
    for line in lines {
        let cur = solve(line.clone());
        println!("Line: {} => {}", line, cur);
        sum += cur;
    }
    println!("Part 1: {}", sum);
}

fn solve(line: String) -> i32 {
    let parts: Vec<&str> = line.split(" ").collect();
    let char_part: &str = parts[0];
    let chars: Vec<char> = char_part.chars().collect();
    let group_list: &str = parts[1];
    let groups: Vec<usize> = group_list.split(",").filter_map(|word| word.parse().ok())
        .collect();
    return solve_rec2(0, false, 0, 0, &groups, &chars, -1, "".to_string());
}

fn solve_rec2(pos: usize, in_row: bool, row_size: usize, cur_count: usize, groups: &Vec<usize>, chars: &Vec<char>, group_index: i32, cur_str: String) -> i32 {
    // println!("Current str: {} ", cur_str);
    // If we've reached the end of the line and we're not in a row, or we are in a row and it's
    // complete, we've found a proper combo!
    if pos == chars.len() && (!in_row || (in_row && cur_count == row_size)) && group_index as usize == groups.len() - 1 {
        // println!("pos = {}, chars.len() = {}, group_index = {}", pos, chars.len(), group_index);
        // println!("Found proper combo: {}", cur_str);
        return 1;
    } else if pos == chars.len() {
        // This could not be a proper combo.
        return 0;
    }
    let cur_char = chars[pos];
    if cur_char == '.' {
        // If the current char is a ',', check if we were in a row, if we were, then make sure we
        // finished that row.
        if in_row {
            if row_size != cur_count {
                println!("broken combo");
                return 0;
            }
        }
        return solve_rec2(pos + 1, false, 0, 0, groups, chars, group_index, cur_str + ".");
    } else if cur_char == '#' {
        // Get the current group if we're already in a row, or the next group if we're not in a row.
        let mut new_group_index = group_index;
        let mut cur_group: usize = 0;
        if in_row {
            cur_group = groups[group_index as usize];
        } else {
            new_group_index = group_index + 1;
            if new_group_index >= groups.len() as i32 {
                return 0;
            }
            cur_group = groups[new_group_index as usize];
        };
        // If we're already at capacity for this row, then we can't recurse any further, this is a broken combo.
        if cur_count == cur_group {
            return 0;
        }
        // Recurse with in_row set to true.
        return solve_rec2(pos + 1, true, cur_group, cur_count + 1, groups, chars, new_group_index, cur_str + "#");
    } else if cur_char == '?' {
        let mut solutions = 0;
        // Try pretending that the current char is a '#', or that the current char is a '.'.
        {
            // Get the current group if we're already in a row, or the next group if we're not in a row.
            let mut can_recurse = true;
            let mut new_group_index = group_index;
            let mut cur_group: usize = 0;
            if in_row {
                cur_group = groups[group_index as usize];
            } else {
                new_group_index = group_index + 1;
                if new_group_index >= groups.len() as i32 {
                    can_recurse = false;
                } else {
                    cur_group = groups[new_group_index as usize];
                }
            };
            // If we're already at capacity for this row, then we can't recurse any further, this is a broken combo.
            if cur_count == cur_group {
                can_recurse = false;
            }
            // Recurse with in_row set to true.
            if can_recurse {
                solutions += solve_rec2(pos + 1, true, cur_group, cur_count + 1, groups, chars, new_group_index, cur_str.clone() + "#");
            }
        }
        {
            let mut can_recurse = true;
            if in_row {
                if row_size != cur_count {
                    // We can't pretend this is a ',', because that would finish a group of the
                    // wrong size.
                    can_recurse = false
                }
            }
            if can_recurse {
                solutions += solve_rec2(pos + 1, false, 0, 0, groups, chars, group_index, cur_str + ".");
            }
        }
        return solutions;
    }
    panic!("Shouldn't get here!");
}