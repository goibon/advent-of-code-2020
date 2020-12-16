use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn split_input(string: &String) -> Vec<&str> {
    let strings: Vec<&str> = string
        .split("\n")
        .collect();
    strings
}

fn parse_operation(operation: &str) -> (&str, i32) {
    let split: Vec<&str> = operation
        .split(" ")
        .map(|string| string.trim())
        .collect();

    let mut command: &str = "";
    let mut amount: i32 = 0;
    if split.len() == 2 {
        if let Ok(number) = split[1].parse() {
            amount = number;
        }
        if split[0] == "acc" || split[0] == "jmp" || split[0] == "nop" {
            command = split[0];
        }
    }
    (command, amount)
}

fn follow_operation(operations: &Vec<&str>) -> i32 {
    let mut visited_operations: HashMap<usize, bool> = HashMap::new();
    let mut next_operation = 0;
    let mut accumulator: i32 = 0;
    while next_operation < operations.len()  {
        let operation = operations[next_operation];
        let (operation, amount) = parse_operation(operation);

        next_operation = match operation {
            "acc" => {
                if let Some(_) = visited_operations.get(&next_operation) {
                    break
                }
                accumulator += amount;
                visited_operations.insert(next_operation, true);
                next_operation + 1
            },
            "jmp" => {
                if let Some(_) = visited_operations.get(&next_operation) {
                    break
                }

                visited_operations.insert(next_operation, true);

                if amount.is_negative() {
                    next_operation - amount.wrapping_abs() as u32 as usize
                } else {
                    next_operation + amount as usize
                }
            },
            "nop" => {
                if let Some(_) = visited_operations.get(&next_operation) {
                    break
                }
                visited_operations.insert(next_operation, true);

                next_operation + 1
            },
            &_ => break,
        }
    }
    accumulator
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path)
        .expect("Error reading file.");
    let input = split_input(&input);

    let part_1 = follow_operation(&input);
    println!("part_1: {:?}", part_1);
}

#[test]
fn test_parse_operation() {
    assert_eq!(parse_operation(""), ("", 0));
    assert_eq!(parse_operation("nop +1"), ("nop", 1));
    assert_eq!(parse_operation("nop -1"), ("nop", -1));
    assert_eq!(parse_operation("jmp +1"), ("jmp", 1));
    assert_eq!(parse_operation("jmp -1"), ("jmp", -1));
    assert_eq!(parse_operation("acc +1"), ("acc", 1));
    assert_eq!(parse_operation("acc -1"), ("acc", -1));
    assert_eq!(parse_operation("unknown +1"), ("", 1));
    assert_eq!(parse_operation("unknown -1"), ("", -1));
}

#[test]
fn test_follow_operation() {
    let test_map: HashMap<usize, bool> = HashMap::new();

    let test_operations: Vec<&str> = vec!["acc +1", "jmp -1"];
    assert_eq!(follow_operation(&test_operations), 1);

    let test_operations: Vec<&str> = vec!["acc +1", "acc +1", "jmp -1"];
    assert_eq!(follow_operation(&test_operations), 2);

    let test_operations: Vec<&str> = vec!["acc +1", "acc -1", "jmp -1"];
    assert_eq!(follow_operation(&test_operations), 0);

    let test_operations: Vec<&str> = vec!["acc -1", "acc -1", "jmp -1"];
    assert_eq!(follow_operation(&test_operations), -2);

    let test_operations: Vec<&str> = vec!["nop +1", "acc +1", "jmp -1"];
    assert_eq!(follow_operation(&test_operations), 1);

    let test_operations: Vec<&str> = vec!["jmp +2", "acc +1", "jmp -2"];
    assert_eq!(follow_operation(&test_operations), 0);
}
