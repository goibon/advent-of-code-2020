use std::env;
use std::fs::File;
use std::io::Read;
use regex::Regex;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn split_input(string: &String) -> Vec<&str> {
    let strings: Vec<&str> = string
        .split("\n\n")
        .collect();
    strings
}

fn count_valid_passports(passports: &Vec<&str>) -> u8 {
    let regex = Regex::new(r"(byr|iyr|eyr|hgt|hcl|ecl|pid)").unwrap();
    let mut count: u8 = 0;
    for line in passports {
        let regex_match = regex.captures_iter(line);
        if regex_match.count() == 7 {
            count += 1;
        }
    }
    count
}

fn count_valid_passports_part2(passports: &Vec<&str>) -> u8 {
    let field_expressions: Vec<Regex> = vec![
        Regex::new(r"byr:(19[2-9][0-9]|200[0-2])").unwrap(),
        Regex::new(r"iyr:(201[0-9]|2020)").unwrap(),
        Regex::new(r"eyr:(202[0-9]|2030)").unwrap(),
        Regex::new(r"hgt:(((1[5-8][0-9]|19[0-3])cm)|((59|6[0-9]|7[0-6])in))").unwrap(),
        Regex::new(r"hcl:#[0-9a-fA-F]{6}").unwrap(),
        Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap(),
        Regex::new(r"pid:\d{9}(\D|$)").unwrap(),
        ];

    let mut valid_passport_count: u8 = 0;
    for line in passports {
        let mut valid_field_count = 0;
        for expression in &field_expressions {
            if expression.is_match(line) {
                valid_field_count += 1;
            }
        }

        if valid_field_count == field_expressions.len() {
            valid_passport_count += 1;
        }
    }
    valid_passport_count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path)
        .expect("Error reading file.");
    let input = split_input(&input);

    let count = count_valid_passports(&input);
    println!("part1: {:?}", count);

    let count = count_valid_passports_part2(&input);
    println!("part2: {:?}", count);
}
