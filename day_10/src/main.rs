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
