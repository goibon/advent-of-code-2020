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

fn follow_operation(operations: &Vec<&str>) -> (i32, bool, Vec<usize>) {
    let mut visited_operations: HashMap<usize, bool> = HashMap::new();
    let mut next_operation = 0;
    let mut accumulator: i32 = 0;
    let mut terminated_properly = true;
    let mut operations_in_order: Vec<usize> = Vec::new();
    while next_operation < operations.len()  {
        if let Some(_) = visited_operations.get(&next_operation) {
            terminated_properly = false;
            break
        }
        visited_operations.insert(next_operation, true);
        operations_in_order.push(next_operation);

        let operation = operations[next_operation];
        let (operation, amount) = parse_operation(operation);

        next_operation = match operation {
            "acc" => {
                accumulator += amount;
                next_operation + 1
            },
            "jmp" => if amount.is_negative() {
                    next_operation - amount.wrapping_abs() as u32 as usize
                } else {
                    next_operation + amount as usize
                },
            "nop" => next_operation + 1,
            &_ => break,
        }
    }
    (accumulator, terminated_properly, operations_in_order)
}

fn alter_operations_until_proper_termination(original_operations: &Vec<&str>) -> i32 {
    let (accumulator, terminated_properly, mut operations_in_order) = follow_operation(&original_operations);
    let mut result: i32 = if terminated_properly { accumulator } else { 0 };
    operations_in_order.reverse();

    for operation_index in operations_in_order {
        let mut temp_operations = original_operations.clone();
        let operation = temp_operations[operation_index];
        let (operation, amount) = parse_operation(operation);
        let mut new_operation = String::from("");
        if operation == "jmp" {
            new_operation = String::from("nop ") + &amount.to_string();
            temp_operations[operation_index] = &new_operation;
        } else if operation == "nop" {
            new_operation = String::from("jmp ") + &amount.to_string();
            temp_operations[operation_index] = &new_operation;
        } else {
            continue
        }

        let (accumulator, terminated_properly, _) = follow_operation(&temp_operations);
        if terminated_properly {
            result = accumulator;
            break
        }
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path)
        .expect("Error reading file.");
    let input = split_input(&input);

    let (part_1, _, _) = follow_operation(&input);
    println!("part_1: {:?}", part_1);

    let part_2 = alter_operations_until_proper_termination(&input);
    println!("part_2: {:?}", part_2);
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
    let test_operations: Vec<&str> = vec!["acc +1", "jmp -1"];
    assert_eq!(follow_operation(&test_operations), (1, false, vec![0, 1]));

    let test_operations: Vec<&str> = vec!["acc +1", "acc +1", "jmp -1"];
    assert_eq!(follow_operation(&test_operations), (2, false, vec![0, 1, 2]));

    let test_operations: Vec<&str> = vec!["acc +1", "acc -1", "jmp -1"];
    assert_eq!(follow_operation(&test_operations), (0, false, vec![0, 1, 2]));

    let test_operations: Vec<&str> = vec!["acc -1", "acc -1", "jmp -1"];
    assert_eq!(follow_operation(&test_operations), (-2, false, vec![0, 1, 2]));

    let test_operations: Vec<&str> = vec!["nop +1", "acc +1", "jmp -1"];
    assert_eq!(follow_operation(&test_operations), (1, false, vec![0, 1, 2]));

    let test_operations: Vec<&str> = vec!["jmp +2", "acc +1", "jmp -2"];
    assert_eq!(follow_operation(&test_operations), (0, false, vec![0, 2]));
}

#[test]
fn test_alter_operations_until_proper_termination() {
    let test_operations: Vec<&str> = vec!["acc +1", "jmp -1"];
    assert_eq!(alter_operations_until_proper_termination(&test_operations), 1);

    let test_operations: Vec<&str> = vec!["acc +1", "acc +1", "jmp -1"];
    assert_eq!(alter_operations_until_proper_termination(&test_operations), 2);

    let test_operations: Vec<&str> = vec!["acc +1", "nop -1", "jmp -1"];
    assert_eq!(alter_operations_until_proper_termination(&test_operations), 1);

    let test_operations: Vec<&str> = vec!["nop +1", "acc +1", "jmp -1"];
    assert_eq!(alter_operations_until_proper_termination(&test_operations), 1);

    let test_operations: Vec<&str> = vec!["jmp +2", "acc +1", "jmp -2"];
    assert_eq!(alter_operations_until_proper_termination(&test_operations), 0);

    let test_operations: Vec<&str> = vec!["nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6"];
    assert_eq!(alter_operations_until_proper_termination(&test_operations), 8);
}
