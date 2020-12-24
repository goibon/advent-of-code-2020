use std::env;
use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn split_input(string: &str) -> (u32, Vec<u32>) {
    let lines = string.split('\n').collect::<Vec<&str>>();
    let earliest_departure = lines[0].parse().unwrap();
    let busses = lines[1]
        .split(',')
        .filter(|entry| entry != &"x")
        .map(|entry| entry.parse().unwrap())
        .collect();
    (earliest_departure, busses)
}

fn find_nearest_busline(earliest_departure: u32, busses: &[u32]) -> Option<(u32, u32)> {
    if busses.is_empty() {
        None
    } else {
        let mut smallest_difference = u32::MAX;
        let mut closest_bus = u32::MAX;
        for bus in busses {
            let difference = bus - (earliest_departure % bus);
            if difference < smallest_difference {
                smallest_difference = difference;
                closest_bus = *bus;
            }
        }
        if closest_bus != u32::MAX {
            Some((closest_bus, smallest_difference))
        } else {
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path).expect("Error reading file.");
    let (earliest_departure, busses) = split_input(&input);

    let (bus_id, wait_time) = find_nearest_busline(earliest_departure, &busses).unwrap();
    let part_1 = bus_id * wait_time;
    println!("Part 1: {:?}", part_1);
}

#[test]
fn test_find_nearest_busline() {
    let earliest_departure = 939;
    let busses = vec![7, 13, 59, 31, 19];
    assert_eq!(
        find_nearest_busline(earliest_departure, &busses),
        Some((59, 5))
    );
}
