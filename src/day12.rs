extern crate queues;

use std::collections::HashMap;
use std::fs;

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input12.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut sum: i64 = 0;
    unsafe {
        for line in lines {
            let cur = solve(line.clone());
            println!("Line: {} => {}", line, cur);
            sum += cur;
        }
    }
    println!("Part 1: {}", sum);
}

unsafe fn solve(line: String) -> i64 {
    let parts: Vec<&str> = line.split(" ").collect();
    let char_part: &str = parts[0];
    let cc: String = vec![char_part].repeat(5).join("?");
    let new_chars= cc.chars().collect();
    let group_list: &str = parts[1];
    let new_group_list = vec![group_list].repeat(5).join(",");
    let groups: Vec<usize> = new_group_list.split(",").filter_map(|word| word.parse().ok())
        .collect();
    // println!("Chars is {}, groups is {}", cc, new_group_list);
    // return 0;
    let mut cache: HashMap<(usize, i32, usize), i64> = HashMap::new();
    return solve_rec2(0, false, 0, 0, &groups, &new_chars, -1, "".to_string(), &mut cache);
}

unsafe fn solve_rec2(pos: usize, in_row: bool, row_size: usize, cur_count: usize, groups: &Vec<usize>, chars: &Vec<char>, group_index: i32, cur_str: String, cache: &mut HashMap<(usize, i32, usize), i64>) -> i64 {
    // println!("Current str: {} ", cur_str);
    if cache.contains_key(&(pos, group_index, cur_count)) {
        return cache[&(pos, group_index, cur_count)];
    }
    // If we've reached the end of the line and we're not in a row, or we are in a row and it's
    // complete, we've found a proper combo!
    if pos == chars.len() && (!in_row || (in_row && cur_count == row_size)) && group_index as usize == groups.len() - 1 {
        // println!("pos = {}, chars.len() = {}, group_index = {}", pos, chars.len(), group_index);
        // println!("Found proper combo: {}", cur_str);
        cache.insert((pos, group_index, cur_count), 1);
        return 1;
    } else if pos == chars.len() {
        // This could not be a proper combo.
        // println!("Broken 1");
        cache.insert((pos, group_index, cur_count), 0);
        return 0;
    }
    let cur_char = chars[pos];
    if cur_char == '.' {
        // If the current char is a ',', check if we were in a row, if we were, then make sure we
        // finished that row.
        if in_row {
            if row_size != cur_count {
                // println!("broken combo");
                // println!("Broken 2");
                cache.insert((pos, group_index, cur_count), 0);
                return 0;
            }
        }
        let result = solve_rec2(pos + 1, false, 0, 0, groups, chars, group_index, cur_str + ".", cache);
        cache.insert((pos, group_index, cur_count), result);
        return result;
    } else if cur_char == '#' {
        // Get the current group if we're already in a row, or the next group if we're not in a row.
        let mut new_group_index = group_index;
        let cur_group: usize;
        if in_row {
            cur_group = groups[group_index as usize];
        } else {
            new_group_index = group_index + 1;
            if new_group_index >= groups.len() as i32 {
                // println!("Broken 3");
                return 0;
            }
            cur_group = groups[new_group_index as usize];
        };
        // If we're already at capacity for this row, then we can't recurse any further, this is a broken combo.
        if cur_count == cur_group {
            // println!("Broken 4");
            cache.insert((pos, group_index, cur_count), 0);
            return 0;
        }
        // Recurse with in_row set to true.
        let result = solve_rec2(pos + 1, true, cur_group, cur_count + 1, groups, chars, new_group_index, cur_str + "#", cache);
        cache.insert((pos, group_index, cur_count), result);
        return result;
    } else if cur_char == '?' {
        let mut solutions: i64 = 0;
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
                    // println!("Broken 5");
                    can_recurse = false;
                } else {
                    cur_group = groups[new_group_index as usize];
                }
            };
            // If we're already at capacity for this row, then we can't recurse any further, this is a broken combo.
            if cur_count == cur_group {
                // println!("Broken 6: cur_count = {}, cur_group = {}", cur_count, cur_group);
                can_recurse = false;
            }
            // Recurse with in_row set to true.
            if can_recurse {
                solutions += solve_rec2(pos + 1, true, cur_group, cur_count + 1, groups, chars, new_group_index, cur_str.clone() + "#", cache);
            }
        }
        {
            let mut can_recurse = true;
            if in_row {
                if row_size != cur_count {
                    // We can't pretend this is a ',', because that would finish a group of the
                    // wrong size.
                    // println!("Broken 7");
                    can_recurse = false
                }
            }
            if can_recurse {
                let next = solve_rec2(pos + 1, false, 0, 0, groups, chars, group_index, cur_str + ".", cache);
                // println!("Adding {} to {}", next, solutions);
                solutions = solutions + next;
            }
        }
        cache.insert((pos, group_index, cur_count), solutions);
        return solutions;
    }
    panic!("Shouldn't get here!");
}