use std::env;
use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn split_string(string: String) -> Option<Vec<String>> {
    let split_string: Vec<String> = string
        .split("\n")
        .map(|token| token.to_string())
        .collect();
    Some(split_string)
}

#[derive(Debug)]
struct PasswordInfo {
    min_occurence: usize,
    max_occurence: usize,
    character: char
}

fn iterate_lines(lines: &Vec<String>) -> Vec<(usize, usize, char, &str)> {
    let mut valid_passwords: Vec<(usize, usize, char, &str)> = Vec::new();
    for line in lines {
        let mut split_string: Vec<&str> = line
            .split(|character| character == '-' || character == ':' || character == ' ')
            .map(|token| token.trim())
            .collect();
        split_string.retain(|&token| !token.is_empty());

        if split_string.len() != 4 {
            continue
        }
        let password_info = PasswordInfo {
            min_occurence: split_string[0].parse()
                .expect("error"),
            max_occurence: split_string[1].parse()
                .expect("error"),
            character: split_string[2].chars()
                .next()
                .expect("Error")
        };
        let password = split_string[3];

        if test_password(&password_info, password) {
            valid_passwords.push((password_info.min_occurence, password_info.max_occurence, password_info.character, password));
        }
    }

    return valid_passwords
}

fn test_password(password_info: &PasswordInfo, password: &str) -> bool {
    let char_count = password
        .chars()
        .filter(|&char| char == password_info.character)
        .count();
    char_count >= password_info.min_occurence && char_count <= password_info.max_occurence
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path)
        .expect("Error reading file.");

    let lines = split_string(input)
        .expect("Error splitting string");

    let result = iterate_lines(&lines);
    println!("{:?}", result.len());
}
