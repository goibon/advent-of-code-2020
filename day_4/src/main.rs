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

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path)
        .expect("Error reading file.");
    let input = split_input(&input);

    let count = count_valid_passports(&input);
    println!("{:?}", count);
}
