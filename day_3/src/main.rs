use std::env;
use std::fs::File;
use std::io::Read;

const START_X: usize = 0;
const START_Y: usize = 0;
const TREE_CHARACTER: char = '#';

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn split_string(string: &String) -> Vec<&str> {
    let strings: Vec<&str> = string
        .split("\n")
        .collect();
    strings
}

// Returned boolean is whether or not a tree was hit:
// true => tree hit
// false => no tree hit
fn move_tobaggo(start_x: usize, start_y: usize, move_x: usize, move_y: usize, map: &Vec<&str>) -> Option<(usize, usize, bool)> {
    if start_y + move_y >= map.len() {
        println!("Illegal move");
        None
    } else {
        let segment_width = map[0].chars().count();
        let new_x = (start_x + move_x) % segment_width;
        let new_y = start_y + move_y;
        let new_target = map[new_y].chars().collect::<Vec<char>>()[new_x];
        Some((new_x, new_y, new_target == TREE_CHARACTER))
    }
}

fn count_trees(move_x: usize, move_y: usize, map: &Vec<&str>) -> u8 {
    let mut count: u8 = 0;
    let mut move_result = move_tobaggo(START_X, START_Y, move_x, move_y, map);

    while let Some((new_x, new_y, tree_hit)) = move_result {
        if tree_hit {
            count += 1;
        }
        if new_y + move_y >= map.len() {
            break
        }
        move_result = move_tobaggo(new_x, new_y, move_x, move_y, map);
    }
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path)
        .expect("Error reading file.");
    let input = split_string(&input);

    let slopes: Vec<(usize, usize)> = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];

    let mut result: Vec<u32> = Vec::new();
    for slope in slopes {
        let count = count_trees(slope.0, slope.1, &input);
        result.push(count.into());
    }

    println!("Result: {:?}", result.iter().product::<u32>());
}
