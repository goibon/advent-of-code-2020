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

fn get_unique_answers(answers: &str) -> Vec<char> {
    let mut answers: Vec<char> = answers
        .chars()
        .filter(|&character| character != '\n')
        .collect();
    answers.sort();
    answers.dedup();
    answers
}

#[test]
fn test_count_unique_answers() {
    assert_eq!(get_unique_answers(""), vec![]);
    assert_eq!(get_unique_answers("a"), vec!['a']);
    assert_eq!(get_unique_answers("aa"), vec!['a']);
    assert_eq!(get_unique_answers("ab"), vec!['a', 'b']);
    assert_eq!(get_unique_answers("aba"), vec!['a', 'b']);
}

fn sum_answers(input: &Vec<&str>) -> u32 {
    let result: u32 = input
        .iter()
        .map(|group| get_unique_answers(group).len() as u32)
        .sum();
    result
}

fn split_group(group: &str) -> Vec<&str> {
    group
        .split("\n")
        .collect::<Vec<&str>>()
}

#[test]
fn test_split_group() {
    assert_eq!(split_group(""), vec![""]);
    assert_eq!(split_group("a\na"), vec!["a", "a"]);
    assert_eq!(split_group("a\nb\na"), vec!["a", "b", "a"]);
    assert_eq!(split_group("ab\na"), vec!["ab", "a"]);
}

fn count_answers(groups: &Vec<&str>) -> u32 {
    let mut result: u32 = 0;
    for group in groups {
        let unique_answers = get_unique_answers(group);
        let group_answers = split_group(group);
        let group_len = group_answers.len();

        for unique_answer in unique_answers {
            let count = group_answers
                .iter()
                .map(|group_answer| group_answer.contains(unique_answer))
                .filter(|&answer| answer == true)
                .count();
            if count == group_len {
                result += 1;
            }
        }
    }

    result
}

#[test]
fn test_count_answers() {
    assert_eq!(count_answers(&vec![""]), 0);
    assert_eq!(count_answers(&vec!["a"]), 1);
    assert_eq!(count_answers(&vec!["aa"]), 1);
    assert_eq!(count_answers(&vec!["ab"]), 2);
    assert_eq!(count_answers(&vec!["a\nb"]), 0);
    assert_eq!(count_answers(&vec!["a\na"]), 1);
    assert_eq!(count_answers(&vec!["ab\na"]), 1);
    assert_eq!(count_answers(&vec!["a\nba"]), 1);
    assert_eq!(count_answers(&vec!["a\na\na"]), 1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path)
        .expect("Error reading file.");
    let input = split_input(&input);
    let result = sum_answers(&input);
    println!("Part 1: {:?}", result);

    let result = count_answers(&input);
    println!("Part 2: {:?}", result);
}
