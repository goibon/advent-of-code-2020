use std::env;
use std::fs::File;
use std::io::Read;

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

fn count_unique_answers(answers: &str) -> u32 {
    if answers.is_empty() {
        0
    } else {
        let mut answers: Vec<char> = answers
            .chars()
            .filter(|&character| character != '\n')
            .collect();
        answers.sort();
        answers.dedup();
        answers.len() as u32
    }
}

#[test]
fn test_count_unique_answers() {
    assert_eq!(count_unique_answers(""), 0);
    assert_eq!(count_unique_answers("a"), 1);
    assert_eq!(count_unique_answers("aa"), 1);
    assert_eq!(count_unique_answers("ab"), 2);
    assert_eq!(count_unique_answers("aba"), 2);
}

fn sum_answers(input: &Vec<&str>) -> u32 {
    let result = input
        .iter()
        .map(|group| count_unique_answers(group))
        .sum();
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path)
        .expect("Error reading file.");
    let input = split_input(&input);
    let result = sum_answers(&input);
    println!("Sum of group answer counts: {:?}", result);
}