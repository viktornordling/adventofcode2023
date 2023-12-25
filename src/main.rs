extern crate queues;

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use ascii::AsciiChar::g;

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input12.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    for line in lines {
        solve(line);
    }
}

fn solve(line: String) {
    let parts = line.split(" ").collect();
    let char_part: String = parts[0];
    let chars: Vec<char> = char_part.chars().collect();
    let group_list = parts[1];
    let groups: Vec<i32> = group_list.filter_map(|word| word.parse().ok())
        .collect();
    solve_rec2(0, false, 0, 0, &groups, &chars, -1);
}

fn solve_rec2(pos: usize, in_row: bool, row_size: usize, cur_count: usize, groups: &Vec<usize>, chars: &Vec<char>, group_index: i32) -> i32 {
    // If we've reached the end of the line and we're not in a row, or we are in a row and it's
    // complete, we've found a proper combo!
    if pos == chars.len() - 1 && !in_row || (in_row && cur_count == row_size) {
        return 1;
    } else if pos == chars.len() {
        // This could not be a proper combo.
        return 0;
    }
    let cur_char = chars.get(&pos).unwrap();
    if cur_char == '.' {
        // If the current char is a ',', check if we were in a row, if we were, then make sure we
        // finished that row.
        if in_row {
            if row_size != cur_count {
                println!("broken combo");
                return 0;
            }
        }
        return solve_rec2(pos + 1, false, 0, 0, groups, chars, group_index);
    } else if cur_char == '#' {
        // Get the current group if we're already in a row, or the next group if we're not in a row.
        let mut new_group_index = group_index;
        let mut cur_group: usize = 0;
        if in_row {
            cur_group = groups[group_index];
        } else {
            new_group_index = group_index + 1;
            if new_group_index >= groups.len() as i32 {
                return 0;
            }
            cur_group = groups[new_group_index];
        };
        // Recurse with in_row set to true.
        return solve_rec2(pos + 1, true, cur_group, cur_count + 1, groups, chars, new_group_index);
    } else if cur_char == '?' {
        let mut solutions = 0;
        // Try pretending that the current char is a '#', or that the current char is a '.'.
        {
            // Get the current group if we're already in a row, or the next group if we're not in a row.
            let mut can_recurse = true;
            let mut new_group_index = group_index;
            let mut cur_group: usize = 0;
            if in_row {
                cur_group = groups[group_index];
            } else {
                new_group_index = group_index + 1;
                if new_group_index >= groups.len() as i32 {
                    can_recurse = false;
                }
                cur_group = groups[new_group_index];
            };
            // Recurse with in_row set to true.
            if can_recurse {
                solutions += solve_rec2(pos + 1, true, cur_group, cur_count + 1, groups, chars, new_group_index);
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
                solutions += solve_rec2(pos + 1, false, 0, 0, groups, chars, group_index);
            }
        }
        return solutions;
    }
    panic!("Shouldn't get here!");
}

fn solve_rec(from_group: usize, groups: &Vec<i32>, from_pos: usize, chars: &Vec<char>) -> i32 {
    if from_pos >= chars.len() && from_group >= groups.len() {
        // If we got to the end of the string and we have no more groups, we've found one proper combo.
        return 1;
    }
    if chars[from_pos] == '.' {
        return solve_rec(from_group, groups, from_pos + 1, chars);
    }
    if chars[from_pos] == '#' {
        let group_fits = group_fits(from_group, groups, from_pos, chars);
        if !group_fits {
            return 0;
        }
        let next_pos = get_next_pos();
        return 1 + solve_rec(from_group + 1, groups, next_pos, chars);
    } else if chars[from_pos] == '?' {
        // Try assuming this char is a '#'
        if group_fits_assuming_cur_pos_is_spring() {
            let next_pos = todo!();
            // Count number of combos assuming current char is a '#'.
            let count1 = 1 + solve_rec(from_group + 1, groups, next_pos, chars);
            // And count number of combos assuming current char is a '.'.
            let count2 = solve_rec(from_group, groups, from_pos + 1, chars);
        } else {
            let count2 = solve_rec(from_group, groups, from_pos + 1, chars);
            return count2;
        }
    }
    panic!("Shouldn't get here!");
}

fn group_fits_assuming_cur_pos_is_spring() -> bool {
    todo!()
}

fn group_fits(from_group: usize, groups: &Vec<i32>, from_pos: usize, chars: &Vec<char>) -> bool {
    if from_group >= groups.len() {
        return false;
    }
    let cur_group = groups.get(from_group).unwrap();
    let mut count = 0;
    let mut cur_pos = from_pos;
    while cur_pos < chars.len() && chars[cur_pos] == '#' {
        cur_pos += 1;
        count += 1;
    }
    return count == cur_group;
}

fn get_next_pos() -> usize {
    todo!()
}