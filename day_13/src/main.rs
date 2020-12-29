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

fn split_input_part_2(string: &str) -> Vec<(usize, i128)> {
    let lines = string.split('\n').collect::<Vec<&str>>();
    let busses = lines[1]
        .split(',')
        .enumerate()
        .filter_map(|(index, entry)| {
            if let Ok(entry) = entry.parse() {
                Some((index, entry))
            } else {
                None
            }
        })
        .collect();
    busses
}

fn find_timestamp(busses: &[(usize, i128)]) -> i128 {
    let (index, bus) = busses[0];
    let mut period_combined = bus as i128;
    let mut phase_combined = index as i128;
    let (_, rest) = busses.split_at(1);
    for (index, bus) in rest {
        let (new_period_combined, new_phase_combined) = combine_phased_rotations(
            period_combined as i128,
            phase_combined,
            *bus as i128,
            *index as i128,
        );
        period_combined = new_period_combined;
        phase_combined = new_phase_combined;
    }

    let group_meets = (-phase_combined).rem_euclid(period_combined);
    let mut x = group_meets;
    while !is_departure_time_valid_for_all(x, rest) {
        x += period_combined;
    }

    x as i128
}

fn combine_phased_rotations(
    a_period: i128,
    a_phase: i128,
    b_period: i128,
    b_phase: i128,
) -> (i128, i128) {
    let (gcd, s, _t) = extended_gcd(a_period, b_period);
    let phase_difference = a_phase - b_phase;
    let pd_mult = phase_difference / gcd;
    let pd_remainder = phase_difference % gcd;
    if pd_remainder != 0 {
        println!("Rotation reference points never synchronize");
        (0, 0)
    } else {
        let combined_period = a_period / gcd * b_period;
        let combined_phase = (a_phase - s * pd_mult * a_period) % combined_period;
        (combined_period, combined_phase)
    }
}

fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    while r != 0 {
        let (quotient, remainder) = (old_r / r, old_r % r);
        let (m, n) = (old_s - s * quotient, old_t - t * quotient);

        old_r = r;
        r = remainder;
        old_s = s;
        s = m;
        old_t = t;
        t = n;
    }
    (old_r, old_s, old_t)
}

fn is_departure_time_valid_for_all(x: i128, busses: &[(usize, i128)]) -> bool {
    for (index, bus) in busses {
        if (x + *index as i128) % bus != 0 {
            return false;
        }
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path).expect("Error reading file.");
    let (earliest_departure, busses) = split_input(&input);

    let (bus_id, wait_time) = find_nearest_busline(earliest_departure, &busses).unwrap();
    let part_1 = bus_id * wait_time;
    println!("Part 1: {:?}", part_1);

    let input = split_input_part_2(&input);
    let part_2 = find_timestamp(&input[..]);
    println!("Part 2: {:?}", part_2);
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

#[test]
fn test_find_timestamp() {
    assert_eq!(find_timestamp(&[(0, 17), (2, 13), (3, 19)]), 3417);
    assert_eq!(find_timestamp(&[(0, 67), (1, 7), (2, 59), (3, 61)]), 754018);
    assert_eq!(find_timestamp(&[(0, 67), (2, 7), (3, 59), (4, 61)]), 779210);
    assert_eq!(
        find_timestamp(&[(0, 67), (1, 7), (3, 59), (4, 61)]),
        1261476
    );
    assert_eq!(
        find_timestamp(&[(0, 1789), (1, 37), (2, 47), (3, 1889)]),
        1202161486
    );
    assert_eq!(
        find_timestamp(&[(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)]),
        1068781
    );
}

#[test]
fn test_extended_gcd() {
    assert_eq!(extended_gcd(180, 150), (30, 1, -1));
}
