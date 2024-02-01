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
        let mut highest_z_under = 0;
        let low_z = min(block.start.z, block.end.z);
        for other_block in &blocks {
            let other_high_z = min(other_block.start.z, other_block.end.z);
            let x_overlaps = ranges_overlap(&Range { start: block.start.x, end: block.end.x }, &Range { start: other_block.start.x, end: other_block.end.x });
            let y_overlaps = ranges_overlap(&Range { start: block.start.y, end: block.end.y }, &Range { start: other_block.start.y, end: other_block.end.y });
            if other_high_z < low_z && x_overlaps && y_overlaps {
                blocks_under.push(other_block);
                if other_high_z > highest_z_under {
                    highest_z_under = other_high_z;
                }
            }
        }
        if block.start.z < block.end.z {
            let block_height = block.end.z - block.start.z;
            block.start.z = highest_z_under;
            block.end.z = block.start.z + block_height;
        }
    }
}

fn create_block(id: char, line: &String) -> Block {
    let parts: Vec<&str> = line.split("~").collect();
    let start_coords: Vec<&str> = parts[0].split(",").collect();
    let end_coords: Vec<&str> = parts[1].split(",").collect();
    let start = Pos { x: start_coords[0].parse().ok().unwrap(), y: start_coords[1].parse().ok().unwrap(), z: start_coords[2].parse().ok().unwrap() };
    let end = Pos { x: end_coords[0].parse().ok().unwrap(), y: end_coords[1].parse().ok().unwrap(), z: end_coords[2].parse().ok().unwrap() };
    return Block { id, start, end };
}

