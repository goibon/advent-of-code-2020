use std::env;
use std::fs::File;
use std::io::Read;
use std::convert::TryInto;

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

fn calculate_seat_id(row: u32, column: u32) -> u32 {
    const ROW_MULTIPLIER: u32 = 8;
    row * ROW_MULTIPLIER + column
}

#[test]
fn test_calculate_seat_id() {
    assert_eq!(calculate_seat_id(0, 1), 1);
    assert_eq!(calculate_seat_id(1, 1), 9);
    assert_eq!(calculate_seat_id(2, 1), 17);
    assert_eq!(calculate_seat_id(1, 0), 8);
    assert_eq!(calculate_seat_id(1, 2), 10);
}

fn find_row_and_column_strings(string: &str) -> (&str, &str) {
    if string.is_empty() {
        ("", "")
    } else {
        if let Some(index) = string.find(|c: char| c == 'R' || c == 'L') {
            if index == 0 {
                ("", string)
            } else {
                string.split_at(index)
            }
        } else {
            (string, "")
        }
    }
}

#[test]
fn test_find_row_and_column_strings() {
    assert_eq!(find_row_and_column_strings(""), ("", ""));
    assert_eq!(find_row_and_column_strings("F"), ("F", ""));
    assert_eq!(find_row_and_column_strings("B"), ("B", ""));
    assert_eq!(find_row_and_column_strings("R"), ("", "R"));
    assert_eq!(find_row_and_column_strings("L"), ("", "L"));
    assert_eq!(find_row_and_column_strings("FR"), ("F", "R"));
    assert_eq!(find_row_and_column_strings("BR"), ("B", "R"));
    assert_eq!(find_row_and_column_strings("FL"), ("F", "L"));
    assert_eq!(find_row_and_column_strings("BL"), ("B", "L"));
    assert_eq!(find_row_and_column_strings("BFFFBBFRRR"), ("BFFFBBF", "RRR"));
    assert_eq!(find_row_and_column_strings("FFFBBBFRRR"), ("FFFBBBF", "RRR"));
    assert_eq!(find_row_and_column_strings("BBFFBBFRLL"), ("BBFFBBF", "RLL"));
}

fn get_max_value(string: &str) -> u32 {
    let power: u32 = string.len().try_into().unwrap();
    const BASE: u32 = 2;
    BASE.pow(power) - 1
}

#[test]
fn test_get_max_value() {
    assert_eq!(get_max_value(""), 0);
    assert_eq!(get_max_value("F"), 1);
    assert_eq!(get_max_value("FF"), 3);
    assert_eq!(get_max_value("FFFFFFF"), 127);
}

fn find_column_or_row(string: &str, lower_bound: u32) -> u32 {
    if string.is_empty() {
        0
    } else {
        let max_value = get_max_value(string) + lower_bound;
        if string.len() == 1 {
            if string == "F" || string == "L" {
                return lower_bound
            } else {
                return max_value
            }
        } else {
            let (first, new_string) = string.split_at(1);
            let lower_bound = if first == "F" || first == "L" {
                lower_bound
            } else {
                (max_value - lower_bound) / 2 + 1 + lower_bound
            };
            find_column_or_row(new_string, lower_bound)
        }
    }
}

#[test]
fn test_find_column_or_row() {
    assert_eq!(find_column_or_row("", 0), 0);
    assert_eq!(find_column_or_row("F", 0), 0);
    assert_eq!(find_column_or_row("B", 0), 1);
    assert_eq!(find_column_or_row("L", 0), 0);
    assert_eq!(find_column_or_row("R", 0), 1);
    assert_eq!(find_column_or_row("FF", 0), 0);
    assert_eq!(find_column_or_row("BB", 0), 3);
    assert_eq!(find_column_or_row("LL", 0), 0);
    assert_eq!(find_column_or_row("RR", 0), 3);
    assert_eq!(find_column_or_row("FB", 0), 1);
    assert_eq!(find_column_or_row("BF", 0), 2);
    assert_eq!(find_column_or_row("LR", 0), 1);
    assert_eq!(find_column_or_row("RL", 0), 2);
    assert_eq!(find_column_or_row("BFFFBBF", 0), 70);
    assert_eq!(find_column_or_row("FFFBBBF", 0), 14);
    assert_eq!(find_column_or_row("BBFFBBF", 0), 102);
}

fn find_highest_seat_id(input: &Vec<&str>) -> u32 {
    let mut highest_seat_id: u32 = 0;
    for entry in input {
        let (row, column) = find_row_and_column_strings(entry);
        let row = find_column_or_row(row, 0);
        let column = find_column_or_row(column, 0);
        let seat_id = calculate_seat_id(row, column);
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    highest_seat_id
}

fn find_missing_seat_id(input: &Vec<&str>) -> u32 {
    let mut seat_ids: Vec<u32> = input
        .iter()
        .map(|entry| {
            let (row, column) = find_row_and_column_strings(entry);
            let row = find_column_or_row(row, 0);
            let column = find_column_or_row(column, 0);
            let seat_id = calculate_seat_id(row, column);
            seat_id
        })
        .collect();
    seat_ids.sort();

    let mut result: u32 = 0;
    for index in 1..seat_ids.len()-1 {
        let previous = seat_ids[index-1];
        let current = seat_ids[index];
        let next = seat_ids[index+1];

        if previous + 1 == current && next - 1 == current {
            continue
        } else {
            let a = current - previous - 1;
            let b = next - current - 1;
            result = if a > b {current - a} else {current + b};
            break
        }
    }

    result
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path)?;
    let input = split_input(&input);

    println!("Highest seat id: {:?}", find_highest_seat_id(&input));
    println!("Missing seat id: {:?}", find_missing_seat_id(&input));

    Ok(())
}
