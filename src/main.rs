use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::ops::Range;

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
    id: char,
    start: Pos,
    end: Pos,
}

fn compare_blocks(b1: &Block, b2: &Block) -> Ordering {
    let max_z_1 = min(b1.start.z, b1.end.z);
    let max_z_2 = min(b2.start.z, b2.end.z);
    return max_z_1.cmp(&max_z_2);
}

fn ranges_overlap(r1: &Range<i32>, r2: &Range<i32>) -> bool {
    return r1.start <= r2.end && r2.start <= r1.end;
}

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input22.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut id = 'A';
    let mut blocks: Vec<Block> = Vec::new();
    for line in lines.iter() {
        let block = create_block(id, &line);
        id = std::char::from_u32((id as u32) + 1).unwrap();
        blocks.push(block);
    }
    // let mut sorted_map: SortedMap<Block, bool>;
    blocks.sort_by(compare_blocks);

    // Drop each block to its final resting position.
    // The blocks are sorted by z, so we'll look at the lowest block first.
    for i in 0..blocks.len() {
        let mut block = blocks[i];
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
            if block.id == 'D' && other_block.id == 'C' {
                println!("!!! c.start.z = {}, c.end.z = {}", other_block.start.z, other_block.end.z);
            }
            if other_high_z < low_z && x_overlaps && y_overlaps {
                blocks_under.push(other_block);
                if other_high_z + 1 > highest_z_under {
                    highest_z_under = other_high_z + 1;
                }
            }
        }
        if block.start.z < block.end.z {
            println!("1 New z for block {} is {}", block.id, highest_z_under);
            let block_height = block.end.z - block.start.z;
            // println!("z.start = {} z.end = {}", block.start.z, block.end.z);
            blocks[i].start.z = highest_z_under;
            blocks[i].end.z = block.start.z + block_height;
        } else {
            println!("2 New z for block {} is {}", block.id, highest_z_under);
            let block_height = block.start.z - block.end.z;
            // println!("block height is {}", block_height);
            // block.end.z = highest_z_under;
            // block.start.z = block.end.z + block_height;
            blocks[i].end.z = highest_z_under;
            blocks[i].start.z = highest_z_under + block_height;
            // println!("z.start = {} z.end = {}", block.start.z, block.end.z);
        }
    }

    let mut block_to_blocks_it_rests_on: HashMap<char, Vec<char>> = HashMap::new();
    let mut block_to_blocks_that_rest_on_it: HashMap<char, Vec<char>> = HashMap::new();

    let mut num_can_be_disintegrated = 0;
    for block in &blocks {
        let mut blocks_i_rest_on: Vec<char> = Vec::new();
        let mut blocks_that_rest_on_me: Vec<char> = Vec::new();
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
                println!("I ({}) rest on block {}", block.id, other_block.id);
                blocks_i_rest_on.push(other_block.id);
            } else {
                println!("I ({}) don't rest on block {}", block.id, other_block.id);
            }
            if other_low_z == this_high_z + 1 && x_overlaps && y_overlaps {
                println!("Block ({}) rests on me ({})", other_block.id, block.id);
                blocks_that_rest_on_me.push(other_block.id);
            } else {
                println!("Block ({}) doesn't rest on me {}", other_block.id, block.id);
            }
        }
        block_to_blocks_it_rests_on.insert(block.id, blocks_i_rest_on.clone());
        block_to_blocks_that_rest_on_it.insert(block.id, blocks_that_rest_on_me.clone());
        if blocks_that_rest_on_me.len() == 0 {
            println!("block {} can be disintegrated!", block.id);
            num_can_be_disintegrated += 1;
        }
    }
    for x in block_to_blocks_that_rest_on_it.iter() {
        if x.1.len() > 0 {
            let mut can_be_disintegrated = true;
            for y in x.1 {
                let bb = block_to_blocks_it_rests_on.get(y).unwrap();
                if bb.len() < 2 {
                    println!("Block {} only rests on this block, we can't disintegrated it! this={}, bb={:?}", y, x.0, bb);
                    can_be_disintegrated = false;
                }
            }
            if can_be_disintegrated {
                num_can_be_disintegrated += 1;
                println!("block {} can be disintegrated!", x.0);
            }
        }
    }
    println!("Num = {}", num_can_be_disintegrated);
}

fn create_block(id: char, line: &String) -> Block {
    let parts: Vec<&str> = line.split("~").collect();
    let start_coords: Vec<&str> = parts[0].split(",").collect();
    let end_coords: Vec<&str> = parts[1].split(",").collect();
    let start = Pos { x: start_coords[0].parse().ok().unwrap(), y: start_coords[1].parse().ok().unwrap(), z: start_coords[2].parse().ok().unwrap() };
    let end = Pos { x: end_coords[0].parse().ok().unwrap(), y: end_coords[1].parse().ok().unwrap(), z: end_coords[2].parse().ok().unwrap() };
    return Block { id, start, end };
}

