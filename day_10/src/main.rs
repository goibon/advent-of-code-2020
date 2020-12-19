use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn split_input(string: &String) -> Vec<u32> {
    string
        .split("\n")
        .map(|token| token.parse().unwrap())
        .collect()
}

fn find_differences(adapters: &Vec<u32>) -> Option<HashMap<u32, u32>> {
    if adapters.is_empty() {
        return None
    }

    let mut map: HashMap<u32, u32> = HashMap::new();
    let mut adapters = adapters.clone();
    adapters.sort();

    let mut previous_value = 0;
    for (index, adapter) in adapters.iter().enumerate() {
        let difference = adapter - previous_value;
        if difference > 3 {
            return None
        }
        if let Some(value) = map.get_mut(&difference) {
            *value += 1;
        } else {
            map.insert(difference, 1);
        }

        previous_value = *adapter;

        if index == adapters.len() - 1 {
            if let Some(value) = map.get_mut(&3) {
                *value += 1;
            } else {
                map.insert(3, 1);
            }
        }
    }

    Some(map)
}

fn find_sets_of_1_joltage_difference(adapters: &Vec<u64>) -> Option<Vec<Vec<u64>>> {
    if adapters.is_empty() {
        return None
    }

    let mut previous_value = 0;
    let mut result: Vec<Vec<u64>> = Vec::new();
    let mut temp_list: Vec<u64> = Vec::new();
    for (index, adapter) in adapters.iter().enumerate() {
        let difference = adapter - previous_value;

        if temp_list.is_empty() {
            temp_list.push(previous_value);
        }
        if difference == 1 {
            temp_list.push(*adapter);
            if index == adapters.len() - 1 {
                result.push(temp_list.clone());
            }
        } else {
            if temp_list.len() > 1 {
                result.push(temp_list);
            }
            temp_list = Vec::new();
        }

        previous_value = *adapter;
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

fn tribonacci(depth: usize) -> Vec<u64> {
    if depth == 0 {
        return vec![]
    } else if depth == 1 {
        return vec![0]
    } else if depth == 2 {
        return vec![0, 1]
    } else if depth == 3 {
        return vec![0, 1, 1]
    }

    let mut result: Vec<u64> = tribonacci(depth-1);
    let (_, rest) = result.split_at(depth-4);
    let new_value = rest.iter().sum();
    result.push(new_value);
    result
}

fn count_permittable_variants(list: &Vec<u64>) -> u64 {
    if list.is_empty() {
        return 0
    } else if list.len() < 3 {
        return 1
    }

    let list = tribonacci(list.len());
    let result = if list.is_empty() {
        0
    } else if list.len() < 3 {
        1
    } else {
        let (_, list) = list.split_at(list.len() - 3);
        list.iter().sum()
    };

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path)
        .expect("Error reading file.");
    let input = split_input(&input);

    let differences = find_differences(&input).unwrap();
    let one_jolts = differences.get(&1).unwrap();
    let three_jolts = differences.get(&3).unwrap();
    let part_1 = one_jolts * three_jolts;
    println!("Part 1: {:?}", part_1);

    let mut input = input.iter().map(|entry| *entry as u64).collect::<Vec<u64>>();
    input.sort();
    let sets_of_1_joltage_difference = find_sets_of_1_joltage_difference(&input).unwrap();
    let mut part_2 = 1;
    for set in sets_of_1_joltage_difference {
        let count = count_permittable_variants(&set);
        part_2 *= count;
    }
    println!("Part 2: {:?}", part_2);
}

#[test]
fn test_find_differences() {
    let mut map: HashMap<u32, u32> = HashMap::new();

    assert_eq!(find_differences(&vec![]), None);
    assert_eq!(find_differences(&vec![4]), None);

    map.insert(1, 1);
    map.insert(3, 1);
    assert_eq!(find_differences(&vec![1]), Some(map));

    map = HashMap::new();
    map.insert(2, 1);
    map.insert(3, 1);
    assert_eq!(find_differences(&vec![2]), Some(map));

    map = HashMap::new();
    map.insert(3, 2);
    assert_eq!(find_differences(&vec![3]), Some(map));

    map = HashMap::new();
    map.insert(1, 2);
    map.insert(3, 1);
    assert_eq!(find_differences(&vec![1, 2]), Some(map));

    map = HashMap::new();
    map.insert(1, 4);
    map.insert(3, 1);
    assert_eq!(find_differences(&vec![1, 2, 3, 4]), Some(map));

    map = HashMap::new();
    map.insert(1, 4);
    map.insert(3, 1);
    assert_eq!(find_differences(&vec![4, 3, 2, 1]), Some(map));

    map = HashMap::new();
    map.insert(1, 4);
    map.insert(3, 1);
    assert_eq!(find_differences(&vec![1, 3, 2, 4]), Some(map));

    map = HashMap::new();
    map.insert(1, 2);
    map.insert(2, 1);
    map.insert(3, 1);
    assert_eq!(find_differences(&vec![1, 2, 4]), Some(map));

    map = HashMap::new();
    map.insert(1, 1);
    map.insert(3, 3);
    assert_eq!(find_differences(&vec![3, 4, 7]), Some(map));

    map = HashMap::new();
    map.insert(1, 7);
    map.insert(3, 5);
    assert_eq!(find_differences(&vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]), Some(map));
}

#[test]
fn test_count_permittable_variants() {
    assert_eq!(count_permittable_variants(&vec![]), 0);
    assert_eq!(count_permittable_variants(&vec![1]), 1);
    assert_eq!(count_permittable_variants(&vec![1, 2]), 1);
    assert_eq!(count_permittable_variants(&vec![1, 2, 3]), 2);
    assert_eq!(count_permittable_variants(&vec![1, 2, 3, 4]), 4);
    assert_eq!(count_permittable_variants(&vec![1, 2, 3, 4, 5]), 7);
    assert_eq!(count_permittable_variants(&vec![1, 2, 3, 4, 5, 6]), 13);
    assert_eq!(count_permittable_variants(&vec![1, 2, 3, 4, 5, 6, 7]), 24);
    assert_eq!(count_permittable_variants(&vec![1, 2, 3, 4, 5, 6, 7, 8]), 44);
}

#[test]
fn test_tribonacci() {
    assert_eq!(tribonacci(0), vec![]);
    assert_eq!(tribonacci(1), vec![0]);
    assert_eq!(tribonacci(2), vec![0, 1]);
    assert_eq!(tribonacci(3), vec![0, 1, 1]);
    assert_eq!(tribonacci(4), vec![0, 1, 1, 2]);
    assert_eq!(tribonacci(5), vec![0, 1, 1, 2, 4]);
}

#[test]
fn test_find_sets_of_1_joltage_difference() {
    assert_eq!(find_sets_of_1_joltage_difference(&vec![]), None);
    assert_eq!(find_sets_of_1_joltage_difference(&vec![3]), None);
    assert_eq!(find_sets_of_1_joltage_difference(&vec![3, 6]), None);
    assert_eq!(find_sets_of_1_joltage_difference(&vec![1]), Some(vec![vec![0, 1]]));
    assert_eq!(find_sets_of_1_joltage_difference(&vec![1, 4]), Some(vec![vec![0, 1]]));
    assert_eq!(find_sets_of_1_joltage_difference(&vec![1, 4, 7]), Some(vec![vec![0, 1]]));
    assert_eq!(find_sets_of_1_joltage_difference(&vec![1, 2]), Some(vec![vec![0, 1, 2]]));
    assert_eq!(find_sets_of_1_joltage_difference(&vec![1, 2, 5, 6]), Some(vec![vec![0, 1, 2], vec![5, 6]]));
    assert_eq!(find_sets_of_1_joltage_difference(&vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19]), Some(vec![vec![0, 1], vec![4, 5, 6, 7], vec![10, 11, 12], vec![15, 16]]));
}
