extern crate queues;

use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::ops::Range;
use queues::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Hash for Pos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Block {
    id: i32,
    start: Pos,
    end: Pos,
}

fn compare_blocks(b1: &Block, b2: &Block) -> Ordering {
    let max_z_1 = max(b1.start.z, b1.end.z);
    let max_z_2 = max(b2.start.z, b2.end.z);
    return max_z_1.cmp(&max_z_2);
}

fn fix_range(range: &Range<i32>) -> Range<i32> {
    if range.start <= range.end {
        return range.clone();
    }
    return Range { start: range.end, end: range.start };
}

fn ranges_overlap(r1: &Range<i32>, r2: &Range<i32>) -> bool {
    let r1_fixed = fix_range(r1);
    let r2_fixed = fix_range(r2);
    return r1_fixed.start <= r2_fixed.end && r2_fixed.start <= r1_fixed.end;
}

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input22.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut id = 1;
    let mut blocks: Vec<Block> = Vec::new();
    for line in lines.iter() {
        let block = create_block(id, &line);
        id += 1;
        blocks.push(block);
    }
    // let mut sorted_map: SortedMap<Block, bool>;
    blocks.sort_by(compare_blocks);

    // Drop each block to its final resting position.
    // The blocks are sorted by z, so we'll look at the lowest block first.
    for i in 0..blocks.len() {
        let block = blocks[i];
        // Find all blocks which are "below" this block.
        // Only count blocks which overlap in the y/x and which have a z which is below this block
        let mut blocks_under: Vec<&Block> = Vec::new();
        let mut highest_z_under = 1;
        let low_z = min(block.start.z, block.end.z);
        for other_block in &blocks {
            let other_high_z = max(other_block.start.z, other_block.end.z);
            // let other_low_z = min(other_block.start.z, other_block.end.z);
            let x_overlaps = ranges_overlap(&Range { start: block.start.x, end: block.end.x }, &Range { start: other_block.start.x, end: other_block.end.x });
            let y_overlaps = ranges_overlap(&Range { start: block.start.y, end: block.end.y }, &Range { start: other_block.start.y, end: other_block.end.y });
            if other_high_z < low_z && x_overlaps && y_overlaps {
                blocks_under.push(other_block);
                if other_high_z + 1 > highest_z_under {
                    highest_z_under = other_high_z + 1;
                }
            }
        }
        if block.start.z < block.end.z {
            let block_height = block.end.z - block.start.z;
            blocks[i].start.z = highest_z_under;
            blocks[i].end.z = blocks[i].start.z + block_height;
        } else {
            let block_height = block.start.z - block.end.z;
            blocks[i].end.z = highest_z_under;
            blocks[i].start.z = highest_z_under + block_height;
        }
    }

    let mut block_to_blocks_it_rests_on: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut block_to_blocks_that_rest_on_it: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut num_can_be_disintegrated = 0;
    for block in &blocks {
        let mut blocks_i_rest_on: Vec<i32> = Vec::new();
        let mut blocks_that_rest_on_me: Vec<i32> = Vec::new();
        for other_block in &blocks {
            if other_block.id == block.id {
                continue;
            }
            let other_low_z = min(other_block.start.z, other_block.end.z);
            let other_high_z = max(other_block.start.z, other_block.end.z);
            let this_low_z = min(block.start.z, block.end.z);
            let this_high_z = max(block.start.z, block.end.z);
            let x_overlaps = ranges_overlap(&Range { start: block.start.x, end: block.end.x }, &Range { start: other_block.start.x, end: other_block.end.x });
            let y_overlaps = ranges_overlap(&Range { start: block.start.y, end: block.end.y }, &Range { start: other_block.start.y, end: other_block.end.y });

            if other_high_z == this_low_z - 1 && x_overlaps && y_overlaps {
                blocks_i_rest_on.push(other_block.id);
            }
            if other_low_z == this_high_z + 1 && x_overlaps && y_overlaps {
                blocks_that_rest_on_me.push(other_block.id);
            }
        }
        block_to_blocks_it_rests_on.insert(block.id, blocks_i_rest_on.clone());
        block_to_blocks_that_rest_on_it.insert(block.id, blocks_that_rest_on_me.clone());
        if blocks_that_rest_on_me.len() == 0 {
            num_can_be_disintegrated += 1;
        }
    }
    for x in block_to_blocks_that_rest_on_it.iter() {
        if x.1.len() > 0 {
            let mut can_be_disintegrated = true;
            for y in x.1 {
                let bb = block_to_blocks_it_rests_on.get(y).unwrap();
                if bb.len() < 2 {
                    can_be_disintegrated = false;
                }
            }
            if can_be_disintegrated {
                num_can_be_disintegrated += 1;
            }
        }
    }
    println!("Part 1: {}", num_can_be_disintegrated);

    let mut total = 0;
    for block in &blocks {
        let falls = count_blocks_that_would_fall_if_this_block_was_disintegrated(block, &block_to_blocks_it_rests_on, &block_to_blocks_that_rest_on_it);
        // println!("{} blocks would fall if {} was disintegrated", falls, block.id);
        total += falls;
    }
    println!("Part 2: {}", total);
}

fn count_blocks_that_would_fall_if_this_block_was_disintegrated(start_block: &Block, block_to_blocks_it_rests_on: &HashMap<i32, Vec<i32>>, block_to_blocks_that_rest_on_it: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut queue: Queue<i32> = Queue::new();
    _ = queue.add(start_block.id);
    let mut blocks_that_would_fall = 0;
    let mut disintegrated: HashSet<i32> = HashSet::new();
    while queue.size() > 0 {
        let block_id = queue.remove().unwrap();
        disintegrated.insert(block_id);
        // println!("Pulled {} from queue {:?}", block_id, queue);

        // For each block that rests on me, if it will fall down as a result of disintegrating this block,
        // add it to the queue.
        let blocks_that_rest_on_me = block_to_blocks_that_rest_on_it.get(&block_id).unwrap();
        for block_that_rests_on_me in blocks_that_rest_on_me {
            let blocks_this_block_rests_on = block_to_blocks_it_rests_on.get(block_that_rests_on_me).unwrap();
            let mut resting = 0;
            for b in blocks_this_block_rests_on {
                if !disintegrated.contains(b) {
                    resting += 1;
                }
            }
            let would_fall = resting == 0;
            if would_fall {
                blocks_that_would_fall += 1;
                _ = queue.add(block_that_rests_on_me.clone());
            }
        }
    }
    return blocks_that_would_fall;
}

fn create_block(id: i32, line: &String) -> Block {
    let parts: Vec<&str> = line.split("~").collect();
    let start_coords: Vec<&str> = parts[0].split(",").collect();
    let end_coords: Vec<&str> = parts[1].split(",").collect();
    let start = Pos { x: start_coords[0].parse().ok().unwrap(), y: start_coords[1].parse().ok().unwrap(), z: start_coords[2].parse().ok().unwrap() };
    let end = Pos { x: end_coords[0].parse().ok().unwrap(), y: end_coords[1].parse().ok().unwrap(), z: end_coords[2].parse().ok().unwrap() };
    return Block { id, start, end };
}

