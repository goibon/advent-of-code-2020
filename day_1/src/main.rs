use std::env;
use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn parse_string(string: String) -> Result<Vec<u32>, std::num::ParseIntError> {
    string
        .split("\n")
        .map(|token| token.parse())
        .collect()
}


fn recurse_numbers(current_numbers: &Vec<u32>, rest_numbers: &Vec<u32>, max_depth: usize) -> Option<Vec<u32>> {
    if current_numbers.len() < max_depth {
        let mut new_rest_numbers = rest_numbers.to_vec();
        loop  {
            if new_rest_numbers.is_empty() {
                return None
            }

            let (first_value, rest) = new_rest_numbers.split_at(1);
            let mut new_current_numbers = current_numbers.to_vec();
            new_current_numbers.extend_from_slice(&first_value);

            new_rest_numbers = rest.to_vec();
            if let Some(result) = recurse_numbers(&new_current_numbers, &new_rest_numbers, max_depth) {
                return Some(result)
            }
        }
    } else {
        if current_numbers.iter().sum::<u32>() == 2020 {
            return Some(current_numbers.to_vec())
        }
        None
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let max_depth = &args[2].parse::<usize>().expect("Error parsing arg");
    let input = read_file(path)
        .expect("Error reading file.");

    let input = parse_string(input)
        .expect("Error parsing string to number");

    let initial_numbers: Vec<u32> = Vec::new();
    if let Some(result) = recurse_numbers(&initial_numbers, &input, *max_depth) {
        println!("{:?}.product() = {:?}", result, result.iter().product::<u32>());
    }
}
