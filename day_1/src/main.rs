use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

const PATH: &str = "input";

fn parse_string(string: String) -> Result<Vec<u32>, std::num::ParseIntError> {
    string
        .split("\n")
        .map(|token| token.parse())
        .collect()
}

fn compare_numbers(a: &u32, b: &u32) -> bool {
    let result = a + b;
    result == 2020
}

fn iterate_numbers(number: &u32, elements: &[u32]) -> (bool, u32, u32) {
    let mut pair_found = false;
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    for entry in elements {
        let is_equal = compare_numbers(number, entry);
        if is_equal {
            pair_found = true;
            a = *number;
            b = *entry;
            break;
        }
    }
    (pair_found, a, b)
}

fn main() {
    let input = read_file(PATH)
        .expect("Error reading file.");

    let input = parse_string(input)
        .expect("Error parsing string to number");

    let mut tuple = input.split_first();
    while let Some((first, elements)) = tuple {
        let (pair_found, a, b) = iterate_numbers(first, &elements);
        if pair_found {
            println!("a: {:?} b: {:?}", a,b);
            println!("result: {:?}", a * b);
            break
        }
        tuple = elements.split_first();
    }
}
