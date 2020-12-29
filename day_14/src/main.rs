use regex::Regex;
use std::collections::HashMap;
use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn split_input(string: &str) -> Vec<&str> {
    string.split('\n').collect::<Vec<&str>>()
}

fn is_mask_line(line: &str) -> bool {
    line.starts_with("mask")
}

fn is_mem_line(line: &str) -> bool {
    line.starts_with("mem")
}

#[derive(Debug, PartialEq)]
struct Mask {
    and_mask: Option<u64>,
    or_mask: Option<u64>,
}

fn parse_mask_line(mask: &str) -> Option<Mask> {
    if !is_mask_line(mask) {
        return None;
    }

    if let Some(mask) = mask.split("= ").nth(1) {
        let chars = mask.chars().rev();
        let mut and_mask = u64::MAX;
        let mut or_mask = 0;
        let mut and_mask_set = false;
        let mut or_mask_set = false;
        for (index, character) in chars.enumerate() {
            if character == 'X' {
                continue;
            } else if character == '0' {
                let pow = 2_u64.pow(index.try_into().unwrap());
                let new_and_mask = u64::MAX ^ pow;
                and_mask &= new_and_mask;
                and_mask_set = true;
            } else if character == '1' {
                let pow = 2_u64.pow(index.try_into().unwrap());
                let new_or_mask = pow;
                or_mask |= new_or_mask;
                or_mask_set = true;
            } else {
                continue;
            }
        }
        if !and_mask_set && !or_mask_set {
            Some(Mask {
                and_mask: None,
                or_mask: None,
            })
        } else if !and_mask_set {
            Some(Mask {
                and_mask: None,
                or_mask: Some(or_mask),
            })
        } else if !or_mask_set {
            Some(Mask {
                and_mask: Some(and_mask),
                or_mask: None,
            })
        } else {
            Some(Mask {
                and_mask: Some(and_mask),
                or_mask: Some(or_mask),
            })
        }
    } else {
        None
    }
}

fn parse_mem_line(mem_line: &str) -> Option<(u64, u64)> {
    if !is_mem_line(mem_line) {
        return None;
    }

    if let Ok(regex) = Regex::new(r"mem\[(\d+)\] = (\d+)$") {
        let mut iter = regex.captures_iter(mem_line);
        if let Some(capture) = iter.next() {
            if capture.len() == 3 {
                let memory_address = capture[1].parse().unwrap();
                let memory_value = capture[2].parse().unwrap();
                return Some((memory_address, memory_value));
            }
        }
        None
    } else {
        None
    }
}

fn convert_value_using_mask(value: u64, mask: &Mask) -> u64 {
    let mut result = value;
    if let Some(and_mask) = mask.and_mask {
        result &= and_mask;
    }
    if let Some(or_mask) = mask.or_mask {
        result |= or_mask;
    }
    result
}

fn initialize_program(instructions: &[&str]) -> u64 {
    let mut memory_map: HashMap<u64, u64> = HashMap::new();
    let mut current_mask: Mask = Mask {
        and_mask: None,
        or_mask: None,
    };

    for instruction in instructions {
        if let Some(mask) = parse_mask_line(instruction) {
            current_mask = mask;
        } else if let Some((memory_address, memory_value)) = parse_mem_line(instruction) {
            let memory_value = convert_value_using_mask(memory_value, &current_mask);
            memory_map.insert(memory_address, memory_value);
        }
    }

    memory_map.values().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path).expect("Error reading file.");
    let input = split_input(&input);

    let part_1 = initialize_program(&input);
    println!("Part 1: {:?}", part_1);
}

const U36_MAX: u64 = 68719476735;

#[test]
fn test_is_mask_line() {
    assert_eq!(
        is_mask_line("mask = 0X11XX1X010X01101000X01X011101100000"),
        true
    );
    assert_eq!(is_mask_line("mem[4634] = 907"), false);
}

#[test]
fn test_is_mem_line() {
    assert_eq!(
        is_mem_line("mask = 0X11XX1X010X01101000X01X011101100000"),
        false
    );
    assert_eq!(is_mem_line("mem[4634] = 907"), true);
}

#[test]
fn test_parse_mask_line() {
    assert_eq!(parse_mask_line("mem[0] = 0"), None);
    assert_eq!(
        parse_mask_line("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"),
        Some(Mask {
            and_mask: None,
            or_mask: None
        })
    );
    assert_eq!(
        parse_mask_line("mask = 000000000000000000000000000000000000"),
        Some(Mask {
            and_mask: Some(u64::MAX - U36_MAX),
            or_mask: None
        })
    );
    assert_eq!(
        parse_mask_line("mask = 111111111111111111111111111111111111"),
        Some(Mask {
            and_mask: None,
            or_mask: Some(U36_MAX)
        })
    );
    assert_eq!(
        parse_mask_line("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX0"),
        Some(Mask {
            and_mask: Some(u64::MAX - 1),
            or_mask: None
        })
    );
    assert_eq!(
        parse_mask_line("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX0X"),
        Some(Mask {
            and_mask: Some(u64::MAX - 2),
            or_mask: None
        })
    );
    assert_eq!(
        parse_mask_line("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX1"),
        Some(Mask {
            and_mask: None,
            or_mask: Some(1)
        })
    );
    assert_eq!(
        parse_mask_line("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX1X"),
        Some(Mask {
            and_mask: None,
            or_mask: Some(2)
        })
    );
    assert_eq!(
        parse_mask_line("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
        Some(Mask {
            and_mask: Some(u64::MAX - 2),
            or_mask: Some(64)
        })
    );
}

#[test]
fn test_parse_mem_line() {
    assert_eq!(parse_mem_line("mem[0] = 0"), Some((0, 0)));
    assert_eq!(parse_mem_line("mem[1] = 0"), Some((1, 0)));
    assert_eq!(parse_mem_line("mem[0] = 1"), Some((0, 1)));
    assert_eq!(parse_mem_line("mem[1] = 1"), Some((1, 1)));
    assert_eq!(parse_mem_line("mem[65535] = 65535"), Some((65535, 65535)));
}

#[test]
fn test_convert_value_using_mask() {
    let none_mask = Mask {
        and_mask: None,
        or_mask: None,
    };
    let zero_mask = Mask {
        and_mask: Some(u64::MAX - U36_MAX),
        or_mask: None,
    };
    let one_mask = Mask {
        and_mask: None,
        or_mask: Some(U36_MAX),
    };
    let mix_mask = Mask {
        and_mask: Some(u64::MAX - 2),
        or_mask: Some(64),
    };
    assert_eq!(convert_value_using_mask(0, &none_mask), 0);
    assert_eq!(convert_value_using_mask(1, &none_mask), 1);
    assert_eq!(convert_value_using_mask(128, &none_mask), 128);

    assert_eq!(convert_value_using_mask(0, &zero_mask), 0);
    assert_eq!(convert_value_using_mask(1, &zero_mask), 0);
    assert_eq!(convert_value_using_mask(128, &zero_mask), 0);

    assert_eq!(convert_value_using_mask(0, &one_mask), 68719476735);
    assert_eq!(convert_value_using_mask(1, &one_mask), 68719476735);
    assert_eq!(convert_value_using_mask(128, &one_mask), 68719476735);

    assert_eq!(convert_value_using_mask(11, &mix_mask), 73);
    assert_eq!(convert_value_using_mask(101, &mix_mask), 101);
    assert_eq!(convert_value_using_mask(0, &mix_mask), 64);
}
