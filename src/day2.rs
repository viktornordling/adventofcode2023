use std::cmp::max;
use std::fs;

fn main() {
    // Part 1.
    let file_path = "/Users/vnordling/RustroverProjects/advent/src/input2.txt";

    let lines: Vec<String> = fs::read_to_string(file_path)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect();  // gather them together into a vector

    let mut game_id = 1;
    let mut sum = 0;
    for line in &lines {
        let mut possible = true;
        let parts1: Vec<&str> = line.split(":").collect();
        let parts2: Vec<&str> = parts1[1].split(";").collect();
        for part in parts2 {
            let num_and_colors: Vec<&str> = part.split(", ").collect();
            for num_and_color in num_and_colors {
                let num_and_color_parts: Vec<&str> = num_and_color.trim().split(" ").collect();
                let num: i32 = num_and_color_parts[0].parse().unwrap();
                let color = num_and_color_parts[1];
                if color == "red" && num > 12 {
                    println!("Impossible {}", game_id);
                    possible = false;
                    break;
                } else if color == "green" && num > 13 {
                    println!("Impossible {}", game_id);
                    possible = false;
                    break;
                } else if color == "blue" && num > 14 {
                    println!("Impossible {}", game_id);
                    possible = false;
                    break;
                }
            }
        }
        if possible {
            sum += game_id;
        }
        game_id += 1;
    }
    println!("Part 1: {}", sum);

    let mut tot = 0;
    for line in &lines {
        let mut possible = true;
        let parts1: Vec<&str> = line.split(":").collect();
        let parts2: Vec<&str> = parts1[1].split(";").collect();
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;
        for part in parts2 {
            let num_and_colors: Vec<&str> = part.split(", ").collect();
            for num_and_color in num_and_colors {
                let num_and_color_parts: Vec<&str> = num_and_color.trim().split(" ").collect();
                let num: i32 = num_and_color_parts[0].parse().unwrap();
                let color = num_and_color_parts[1];
                if color == "red" {
                    red_max = max(num, red_max);
                } else if color == "green" {
                    green_max = max(num, green_max);
                } else if color == "blue" {
                    blue_max = max(num, blue_max);
                }
            }
        }
        let min_possible = red_max + green_max + blue_max;
        let power = red_max * green_max * blue_max;
        tot += power;
    }
    println!("Part 2: {}", tot);
}
