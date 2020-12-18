use std::env;
use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn split_input(string: &String) -> Vec<u64> {
    string
        .split("\n")
        .map(|token| token.parse().unwrap())
        .collect()
}

fn get_preamble(list: &Vec<u64>, start_index: usize, preamble_length: usize) -> Vec<u64> {
    if list.is_empty() || list.len() < (preamble_length + start_index){
        return vec![]
    }

    let (_, rest) = list.split_at(start_index);
    if rest.len() > preamble_length {
        let (first, _) = rest.split_at(preamble_length);
        first.to_vec()
    } else {
        rest.to_vec()
    }
}

fn find_numbers_that_sum_to(target: u64, numbers: &Vec<u64>) -> Option<(u64, u64)> {
    if numbers.len() < 2 {
        return None
    }

    for i in 0..numbers.len()-1 {
        let x = numbers[i];
        for j in i+1..numbers.len() {
            let y = numbers[j];
            if x + y == target {
                return Some((x, y))
            }
        }
    }
    None
}

fn find_invalid_number(numbers: &Vec<u64>, preamble_length: usize) -> Option<u64> {
    if numbers.len() < preamble_length + 1 {
        return None
    }
    let mut preamble_index: usize = 0;

    for number_index in preamble_length..numbers.len() {
        let preamble = get_preamble(numbers, preamble_index, preamble_length);
        let number = numbers[number_index];
        if preamble.is_empty() {
            return None
        }

        if let Some(_) = find_numbers_that_sum_to(number, &preamble) {
            preamble_index += 1;
        } else {
            return Some(number)
        }
    }
    None
}

fn find_smallest_and_large_number(list: &Vec<u64>) -> Option<(u64, u64)> {
    if list.len() < 2 {
        None
    } else {
        let mut list = list.clone();
        list.sort();
        Some((list[0], list[list.len() - 1]))
    }
}

fn find_contiguous_numbers_that_sum_to(target: u64, list: &Vec<u64>) -> Option<Vec<u64>> {
    if list.len() < 2 {
        return None
    }
    let mut contiguous_list: Vec<u64> = Vec::new();
    let mut start_index: usize = 0;

    let found_list = 'outer: loop {
        if start_index >= list.len() - 1 {
            break false
        }

        contiguous_list = Vec::new();
        let mut temp_result: u64 = 0;

        for index in start_index..list.len() {
            let number = list[index];
            temp_result += number;
            contiguous_list.push(number);

            if temp_result == target && contiguous_list.len() >= 2 {
                break 'outer true
            } else if temp_result > target {
                break
            }
        }

        start_index += 1
    };

    if found_list {
        Some(contiguous_list)
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path)
        .expect("Error reading file.");
    let input = split_input(&input);

    let part_1 = find_invalid_number(&input, 25).unwrap();
    println!("part 1: {:?}", part_1);

    let contiguous_list = find_contiguous_numbers_that_sum_to(part_1, &input).unwrap();
    let (smallest, largest) = find_smallest_and_large_number(&contiguous_list).unwrap();
    let part_2 = smallest + largest;
    println!("part 2: {:?}", part_2);
}

#[test]
fn test_get_preamble() {
    assert_eq!(get_preamble(&vec![], 0, 0), vec![]);
    assert_eq!(get_preamble(&vec![], 1, 0), vec![]);
    assert_eq!(get_preamble(&vec![], 0, 1), vec![]);
    assert_eq!(get_preamble(&vec![1], 0, 0), vec![]);
    assert_eq!(get_preamble(&vec![1], 0, 1), vec![1]);
    assert_eq!(get_preamble(&vec![1, 2, 3], 0, 2), vec![1, 2]);
    assert_eq!(get_preamble(&vec![1, 2, 3], 1, 2), vec![2, 3]);
    assert_eq!(get_preamble(&vec![1, 2, 3, 4], 0, 2), vec![1, 2]);
    assert_eq!(get_preamble(&(1..10).collect(), 4, 4), vec![5, 6, 7, 8]);
    assert_eq!(get_preamble(&(1..10).collect(), 4, 5), vec![5, 6, 7, 8, 9]);
}

#[test]
fn test_find_numbers_that_sum_to() {
    assert_eq!(find_numbers_that_sum_to(0, &vec![]), None);
    assert_eq!(find_numbers_that_sum_to(1, &vec![]), None);
    assert_eq!(find_numbers_that_sum_to(0, &vec![1, 1]), None);
    assert_eq!(find_numbers_that_sum_to(2, &vec![1, 1]), Some((1, 1)));
    assert_eq!(find_numbers_that_sum_to(10, &(1..10).collect()), Some((1, 9)));
}

#[test]
fn test_find_invalid_number() {
    assert_eq!(find_invalid_number(&vec![], 0), None);
    assert_eq!(find_invalid_number(&vec![], 1), None);
    assert_eq!(find_invalid_number(&vec![1, 2], 0), None);
    assert_eq!(find_invalid_number(&vec![1, 2], 1), Some(2));
    assert_eq!(find_invalid_number(&vec![1, 2], 2), None);
    assert_eq!(find_invalid_number(&vec![1, 2, 3], 2), None);
    assert_eq!(find_invalid_number(&vec![1, 2, 3, 4], 2), Some(4));
    assert_eq!(find_invalid_number(&vec![1, 2, 3, 4, 5], 2), Some(4));
    assert_eq!(find_invalid_number(&vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576], 5), Some(127));
}

#[test]
fn test_find_smallest_and_large_number() {
    assert_eq!(find_smallest_and_large_number(&vec![]), None);
    assert_eq!(find_smallest_and_large_number(&vec![1]), None);
    assert_eq!(find_smallest_and_large_number(&vec![1, 1]), Some((1, 1)));
    assert_eq!(find_smallest_and_large_number(&vec![1, 2]), Some((1, 2)));
    assert_eq!(find_smallest_and_large_number(&vec![2, 1]), Some((1, 2)));
    assert_eq!(find_smallest_and_large_number(&vec![1, 2, 3]), Some((1, 3)));
    assert_eq!(find_smallest_and_large_number(&vec![1, 3, 2]), Some((1, 3)));
    assert_eq!(find_smallest_and_large_number(&vec![2, 1, 3]), Some((1, 3)));
    assert_eq!(find_smallest_and_large_number(&vec![2, 3, 1]), Some((1, 3)));
    assert_eq!(find_smallest_and_large_number(&vec![3, 1, 2]), Some((1, 3)));
    assert_eq!(find_smallest_and_large_number(&vec![3, 2, 1]), Some((1, 3)));
}

#[test]
fn test_find_contiguous_numbers_that_sum_to() {
    assert_eq!(find_contiguous_numbers_that_sum_to(0, &vec![]), None);
    assert_eq!(find_contiguous_numbers_that_sum_to(0, &vec![1]), None);
    assert_eq!(find_contiguous_numbers_that_sum_to(0, &vec![1, 2]), None);
    assert_eq!(find_contiguous_numbers_that_sum_to(1, &vec![1]), None);
    assert_eq!(find_contiguous_numbers_that_sum_to(2, &vec![1, 2, 1]), None);
    assert_eq!(find_contiguous_numbers_that_sum_to(2, &vec![1, 1]), Some(vec![1, 1]));
    assert_eq!(find_contiguous_numbers_that_sum_to(2, &vec![0, 1, 1]), Some(vec![0, 1, 1]));
    assert_eq!(find_contiguous_numbers_that_sum_to(2, &vec![0, 1, 1, 0]), Some(vec![0, 1, 1]));
    assert_eq!(find_contiguous_numbers_that_sum_to(2, &vec![2, 1, 1]), Some(vec![1, 1]));
    assert_eq!(find_contiguous_numbers_that_sum_to(2, &vec![2, 1, 1, 2]), Some(vec![1, 1]));
    assert_eq!(find_contiguous_numbers_that_sum_to(4, &vec![1, 1, 2]), Some(vec![1, 1, 2]));
}
