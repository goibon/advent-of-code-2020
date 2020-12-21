use std::env;
use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn split_input(string: &String) -> Vec<Vec<char>> {
    string
        .split("\n")
        .map(|token| token.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

const FLOOR_CHARACTER: char = '.';
const EMPTY_SEAT_CHARACTER: char = 'L';
const OCCUPIED_SEAT_CHARACTER: char = '#';

fn find_surrounding_indices(
    center_x: usize,
    center_y: usize,
    max_width: usize,
    max_height: usize,
) -> Vec<(usize, usize)> {
    if center_x >= max_width || center_y >= max_height {
        return vec![];
    }

    let min_x = if center_x == 0 { 0 } else { center_x - 1 };
    let max_x = if center_x == max_width - 1 {
        center_x
    } else {
        center_x + 1
    };

    let min_y = if center_y == 0 { 0 } else { center_y - 1 };
    let max_y = if center_y == max_height - 1 {
        center_y
    } else {
        center_y + 1
    };

    let mut result: Vec<(usize, usize)> = Vec::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if x == center_x && y == center_y {
                continue;
            }
            result.push((x, y));
        }
    }
    result
}

fn should_empty_seat_convert(
    surrounding_indices: &Vec<(usize, usize)>,
    map: &Vec<Vec<char>>,
) -> bool {
    for (x, y) in surrounding_indices {
        if map[*y][*x] == OCCUPIED_SEAT_CHARACTER {
            return false;
        }
    }
    true
}

fn should_occupied_seat_convert(
    surrounding_indices: &Vec<(usize, usize)>,
    map: &Vec<Vec<char>>,
) -> bool {
    let mut occupied_seat_count = 0;
    for (x, y) in surrounding_indices {
        if map[*y][*x] == OCCUPIED_SEAT_CHARACTER {
            occupied_seat_count += 1;
        }
        if occupied_seat_count >= 4 {
            break;
        }
    }
    occupied_seat_count >= 4
}

fn should_convert(center_x: usize, center_y: usize, map: &Vec<Vec<char>>) -> bool {
    if map.is_empty() || center_y >= map.len() || center_x >= map[0].len() {
        return false;
    }

    let center_char = map[center_y][center_x];
    if center_char == FLOOR_CHARACTER {
        return false;
    }

    let surrounding_indices = find_surrounding_indices(center_x, center_y, map[0].len(), map.len());
    if center_char == EMPTY_SEAT_CHARACTER {
        should_empty_seat_convert(&surrounding_indices, &map)
    } else {
        should_occupied_seat_convert(&surrounding_indices, &map)
    }
}

fn is_change_valid(
    center_x: usize,
    center_y: usize,
    change_x: i8,
    change_y: i8,
    map: &Vec<Vec<char>>,
) -> bool {
    if map.is_empty() {
        return false;
    }

    if (change_x == 0 && change_y == 0)
        || (change_y == -1 && center_y == 0)
        || (change_y == 1 && center_y == map.len() - 1)
        || (change_x == -1 && center_x == 0)
        || (change_x == 1 && center_x == map[center_y].len() - 1)
    {
        false
    } else {
        true
    }
}

fn find_first_in_direction(
    center_x: usize,
    center_y: usize,
    change_x: i8,
    change_y: i8,
    map: &Vec<Vec<char>>,
) -> Option<char> {
    if map.is_empty() || !is_change_valid(center_x, center_y, change_x, change_y, &map) {
        return None;
    }

    let mut found_char = FLOOR_CHARACTER;
    let mut previous_x = center_x;
    let mut previous_y = center_y;

    while found_char == FLOOR_CHARACTER
        && is_change_valid(previous_x, previous_y, change_x, change_y, &map)
    {
        let new_x = match change_x {
            1 => previous_x + 1,
            -1 => previous_x - 1,
            0 => previous_x,
            _ => {
                println!("invalid change");
                0
            }
        };
        let new_y = match change_y {
            1 => previous_y + 1,
            -1 => previous_y - 1,
            0 => previous_y,
            _ => {
                println!("invalid change");
                0
            }
        };

        found_char = map[new_y][new_x];
        previous_x = new_x;
        previous_y = new_y;
        if found_char != FLOOR_CHARACTER {
            return Some(found_char);
        }
    }
    None
}

const ALL_DIRECTIONS: &'static [(i8, i8)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn should_convert_part_2(center_x: usize, center_y: usize, map: &Vec<Vec<char>>) -> bool {
    if map.is_empty() || center_y >= map.len() || center_x >= map[center_y].len() {
        return false;
    }

    let center_char = map[center_y][center_x];
    if center_char == FLOOR_CHARACTER {
        return false;
    }

    if center_char == EMPTY_SEAT_CHARACTER {
        for (x, y) in ALL_DIRECTIONS {
            if let Some(seat) = find_first_in_direction(center_x, center_y, *x, *y, &map) {
                if seat == OCCUPIED_SEAT_CHARACTER {
                    return false;
                }
            }
        }
        true
    } else {
        let mut occupied_seat_count = 0;
        for (x, y) in ALL_DIRECTIONS {
            if let Some(seat) = find_first_in_direction(center_x, center_y, *x, *y, &map) {
                if seat == OCCUPIED_SEAT_CHARACTER {
                    occupied_seat_count += 1;
                }
                if occupied_seat_count >= 5 {
                    return true;
                }
            }
        }
        false
    }
}

fn update_seats(fields_to_update: &Vec<(usize, usize)>, map: &mut Vec<Vec<char>>) {
    for (x, y) in fields_to_update {
        let character = map[*y][*x];
        if character == EMPTY_SEAT_CHARACTER {
            map[*y][*x] = OCCUPIED_SEAT_CHARACTER;
        } else {
            map[*y][*x] = EMPTY_SEAT_CHARACTER;
        }
    }
}

fn find_and_update(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut mutable_map: Vec<Vec<char>> = map.clone();
    let mut indices_to_update: Vec<(usize, usize)> = Vec::new();
    let mut did_map_mutate: bool = true;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if should_convert(x, y, &map) {
                indices_to_update.push((x, y));
                did_map_mutate = true;
            }
        }
    }
    update_seats(&indices_to_update, &mut mutable_map);
    while did_map_mutate {
        let mut new_indices_to_update: Vec<(usize, usize)> = Vec::new();
        did_map_mutate = false;

        for (x, y) in indices_to_update {
            if should_convert(x, y, &mutable_map) {
                new_indices_to_update.push((x, y));
                did_map_mutate = true;
            }
        }
        update_seats(&new_indices_to_update, &mut mutable_map);
        indices_to_update = new_indices_to_update;
    }

    mutable_map
}

fn find_and_update_part_2(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut mutable_map: Vec<Vec<char>> = map.clone();
    let mut indices_to_update = find_all_seats_of_type(EMPTY_SEAT_CHARACTER, &mutable_map);
    let mut did_map_mutate = true;

    update_seats(&indices_to_update, &mut mutable_map);
    while did_map_mutate {
        let mut new_indices_to_update: Vec<(usize, usize)> = Vec::new();
        did_map_mutate = false;

        for (x, y) in indices_to_update {
            if should_convert_part_2(x, y, &mutable_map) {
                new_indices_to_update.push((x, y));
                did_map_mutate = true;
            }
        }
        update_seats(&new_indices_to_update, &mut mutable_map);
        indices_to_update = new_indices_to_update;
    }

    mutable_map
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path).expect("Error reading file.");
    let input = split_input(&input);
    let part_1 = find_and_update(&input);
    let part_1 = count_seats_of_type(OCCUPIED_SEAT_CHARACTER, &part_1);
    println!("Part 1: {:?}", part_1);

    let part_2 = find_and_update_part_2(&input);
    let part_2 = count_seats_of_type(OCCUPIED_SEAT_CHARACTER, &part_2);
    println!("Part 2: {:?}", part_2);
}

fn find_all_seats_of_type(seat_type: char, map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut seats: Vec<(usize, usize)> = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == seat_type {
                seats.push((x, y));
            }
        }
    }
    seats
}

fn count_seats_of_type(seat_type: char, map: &Vec<Vec<char>>) -> u32 {
    find_all_seats_of_type(seat_type, map).len() as u32
}

#[test]
fn test_count_seats_of_type() {
    let floor_map = vec![
        vec!['.', '.', '.'],
        vec!['.', '.', '.'],
        vec!['.', '.', '.'],
    ];
    let empty_seats_map = vec![
        vec!['L', 'L', 'L'],
        vec!['L', 'L', 'L'],
        vec!['L', 'L', 'L'],
    ];
    let occupied_seats_map = vec![
        vec!['#', '#', '#'],
        vec!['#', '#', '#'],
        vec!['#', '#', '#'],
    ];
    let mix_map = vec![
        vec!['.', 'L', '.'],
        vec!['.', 'L', 'L'],
        vec!['L', 'L', '#'],
    ];

    assert_eq!(count_seats_of_type(FLOOR_CHARACTER, &floor_map), 9);
    assert_eq!(count_seats_of_type(FLOOR_CHARACTER, &empty_seats_map), 0);
    assert_eq!(count_seats_of_type(FLOOR_CHARACTER, &occupied_seats_map), 0);
    assert_eq!(count_seats_of_type(FLOOR_CHARACTER, &mix_map), 3);
    assert_eq!(count_seats_of_type(EMPTY_SEAT_CHARACTER, &floor_map), 0);
    assert_eq!(
        count_seats_of_type(EMPTY_SEAT_CHARACTER, &empty_seats_map),
        9
    );
    assert_eq!(
        count_seats_of_type(EMPTY_SEAT_CHARACTER, &occupied_seats_map),
        0
    );
    assert_eq!(count_seats_of_type(EMPTY_SEAT_CHARACTER, &mix_map), 5);
    assert_eq!(count_seats_of_type(OCCUPIED_SEAT_CHARACTER, &floor_map), 0);
    assert_eq!(
        count_seats_of_type(OCCUPIED_SEAT_CHARACTER, &empty_seats_map),
        0
    );
    assert_eq!(
        count_seats_of_type(OCCUPIED_SEAT_CHARACTER, &occupied_seats_map),
        9
    );
    assert_eq!(count_seats_of_type(OCCUPIED_SEAT_CHARACTER, &mix_map), 1);
}

#[test]
fn test_find_surrounding_indices() {
    assert_eq!(find_surrounding_indices(0, 0, 0, 0), vec![]);
    assert_eq!(find_surrounding_indices(1, 0, 0, 0), vec![]);
    assert_eq!(find_surrounding_indices(0, 1, 0, 0), vec![]);
    assert_eq!(find_surrounding_indices(1, 1, 0, 0), vec![]);
    assert_eq!(find_surrounding_indices(1, 0, 1, 0), vec![]);
    assert_eq!(find_surrounding_indices(1, 0, 0, 1), vec![]);
    assert_eq!(find_surrounding_indices(0, 1, 1, 0), vec![]);
    assert_eq!(find_surrounding_indices(0, 1, 0, 1), vec![]);
    assert_eq!(find_surrounding_indices(0, 0, 1, 1), vec![]);
    assert_eq!(find_surrounding_indices(0, 0, 2, 1), vec![(1, 0)]);
    assert_eq!(find_surrounding_indices(0, 0, 1, 2), vec![(0, 1)]);
    assert_eq!(find_surrounding_indices(1, 0, 2, 1), vec![(0, 0)]);
    assert_eq!(find_surrounding_indices(0, 1, 1, 2), vec![(0, 0)]);
    assert_eq!(
        find_surrounding_indices(0, 0, 2, 2),
        vec![(1, 0), (0, 1), (1, 1)]
    );
    assert_eq!(
        find_surrounding_indices(1, 0, 2, 2),
        vec![(0, 0), (0, 1), (1, 1)]
    );
    assert_eq!(
        find_surrounding_indices(0, 1, 2, 2),
        vec![(0, 0), (1, 0), (1, 1)]
    );
    assert_eq!(
        find_surrounding_indices(1, 1, 2, 2),
        vec![(0, 0), (1, 0), (0, 1)]
    );
    assert_eq!(
        find_surrounding_indices(1, 1, 3, 3),
        vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2)
        ]
    );
}

#[test]
fn test_should_convert() {
    let floor_map = vec![
        vec!['.', '.', '.'],
        vec!['.', '.', '.'],
        vec!['.', '.', '.'],
    ];
    let empty_seats_map = vec![
        vec!['L', 'L', 'L'],
        vec!['L', 'L', 'L'],
        vec!['L', 'L', 'L'],
    ];
    let occupied_seats_map = vec![
        vec!['#', '#', '#'],
        vec!['#', '#', '#'],
        vec!['#', '#', '#'],
    ];
    assert_eq!(should_convert(0, 0, &vec![]), false);

    assert_eq!(should_convert(0, 0, &floor_map), false);
    assert_eq!(should_convert(1, 0, &floor_map), false);
    assert_eq!(should_convert(2, 0, &floor_map), false);
    assert_eq!(should_convert(0, 1, &floor_map), false);
    assert_eq!(should_convert(1, 1, &floor_map), false);
    assert_eq!(should_convert(2, 1, &floor_map), false);
    assert_eq!(should_convert(0, 2, &floor_map), false);
    assert_eq!(should_convert(1, 2, &floor_map), false);
    assert_eq!(should_convert(2, 2, &floor_map), false);

    assert_eq!(should_convert(0, 0, &empty_seats_map), true);
    assert_eq!(should_convert(1, 0, &empty_seats_map), true);
    assert_eq!(should_convert(2, 0, &empty_seats_map), true);
    assert_eq!(should_convert(0, 1, &empty_seats_map), true);
    assert_eq!(should_convert(1, 1, &empty_seats_map), true);
    assert_eq!(should_convert(2, 1, &empty_seats_map), true);
    assert_eq!(should_convert(0, 2, &empty_seats_map), true);
    assert_eq!(should_convert(1, 2, &empty_seats_map), true);
    assert_eq!(should_convert(2, 2, &empty_seats_map), true);

    assert_eq!(should_convert(0, 0, &occupied_seats_map), false);
    assert_eq!(should_convert(1, 0, &occupied_seats_map), true);
    assert_eq!(should_convert(2, 0, &occupied_seats_map), false);
    assert_eq!(should_convert(0, 1, &occupied_seats_map), true);
    assert_eq!(should_convert(1, 1, &occupied_seats_map), true);
    assert_eq!(should_convert(2, 1, &occupied_seats_map), true);
    assert_eq!(should_convert(0, 2, &occupied_seats_map), false);
    assert_eq!(should_convert(1, 2, &occupied_seats_map), true);
    assert_eq!(should_convert(2, 2, &occupied_seats_map), false);

    let mix_map = vec![
        vec!['.', 'L', '.'],
        vec!['.', 'L', 'L'],
        vec!['L', 'L', '#'],
    ];
    assert_eq!(should_convert(0, 0, &mix_map), false);
    assert_eq!(should_convert(1, 0, &mix_map), true);
    assert_eq!(should_convert(2, 0, &mix_map), false);
    assert_eq!(should_convert(0, 1, &mix_map), false);
    assert_eq!(should_convert(1, 1, &mix_map), false);
    assert_eq!(should_convert(2, 1, &mix_map), false);
    assert_eq!(should_convert(0, 2, &mix_map), true);
    assert_eq!(should_convert(1, 2, &mix_map), false);
    assert_eq!(should_convert(2, 2, &mix_map), false);
}

#[test]
fn test_is_change_valid() {
    let test_map = vec![
        vec!['.', '.', '.'],
        vec!['.', '.', '.'],
        vec!['.', '.', '.'],
    ];

    assert!(!is_change_valid(0, 0, 0, 0, &test_map));
    assert!(!is_change_valid(2, 0, 0, 0, &test_map));
    assert!(!is_change_valid(1, 1, 0, 0, &test_map));
    assert!(!is_change_valid(0, 2, 0, 0, &test_map));
    assert!(!is_change_valid(2, 2, 0, 0, &test_map));

    assert!(!is_change_valid(0, 0, -1, -1, &test_map));
    assert!(!is_change_valid(1, 0, -1, -1, &test_map));
    assert!(!is_change_valid(2, 0, -1, -1, &test_map));
    assert!(!is_change_valid(0, 1, -1, -1, &test_map));
    assert!(is_change_valid(1, 1, -1, -1, &test_map));
    assert!(is_change_valid(2, 1, -1, -1, &test_map));
    assert!(!is_change_valid(0, 2, -1, -1, &test_map));
    assert!(is_change_valid(1, 2, -1, -1, &test_map));
    assert!(is_change_valid(2, 2, -1, -1, &test_map));

    assert!(is_change_valid(0, 0, 1, 1, &test_map));
    assert!(is_change_valid(1, 0, 1, 1, &test_map));
    assert!(!is_change_valid(2, 0, 1, 1, &test_map));
    assert!(is_change_valid(0, 1, 1, 1, &test_map));
    assert!(is_change_valid(1, 1, 1, 1, &test_map));
    assert!(!is_change_valid(2, 1, 1, 1, &test_map));
    assert!(!is_change_valid(0, 2, 1, 1, &test_map));
    assert!(!is_change_valid(1, 2, 1, 1, &test_map));
    assert!(!is_change_valid(2, 2, 1, 1, &test_map));
}

#[test]
fn test_find_first_in_direction() {
    let floor_map = vec![
        vec!['.', '.', '.'],
        vec!['.', '.', '.'],
        vec!['.', '.', '.'],
    ];
    let mix_map = vec![
        vec!['.', 'L', '.'],
        vec!['.', 'L', 'L'],
        vec!['L', 'L', '#'],
    ];

    let discover_all_occupied_map = vec![
        vec!['.', '.', '.', '.', '.', '.', '.', '#', '.'],
        vec!['.', '.', '.', '#', '.', '.', '.', '.', '.'],
        vec!['.', '#', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '#', 'L', '.', '.', '.', '.', '#'],
        vec!['.', '.', '.', '.', '#', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['#', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '#', '.', '.', '.', '.', '.'],
    ];

    assert_eq!(find_first_in_direction(0, 0, -1, -1, &floor_map), None);
    assert_eq!(find_first_in_direction(0, 0, 0, -1, &floor_map), None);
    assert_eq!(find_first_in_direction(0, 0, 1, -1, &floor_map), None);
    assert_eq!(find_first_in_direction(0, 0, -1, 0, &floor_map), None);
    assert_eq!(find_first_in_direction(0, 0, 0, 0, &floor_map), None);
    assert_eq!(find_first_in_direction(0, 0, 1, 0, &floor_map), None);
    assert_eq!(find_first_in_direction(0, 0, -1, 1, &floor_map), None);
    assert_eq!(find_first_in_direction(0, 0, 0, 1, &floor_map), None);
    assert_eq!(find_first_in_direction(0, 0, 1, 1, &floor_map), None);

    assert_eq!(find_first_in_direction(1, 1, -1, -1, &mix_map), None);
    assert_eq!(
        find_first_in_direction(1, 1, 0, -1, &mix_map),
        Some(EMPTY_SEAT_CHARACTER)
    );
    assert_eq!(find_first_in_direction(1, 1, 1, -1, &mix_map), None);
    assert_eq!(find_first_in_direction(1, 1, -1, 0, &mix_map), None);
    assert_eq!(find_first_in_direction(1, 1, 0, 0, &mix_map), None);
    assert_eq!(
        find_first_in_direction(1, 1, 1, 0, &mix_map),
        Some(EMPTY_SEAT_CHARACTER)
    );
    assert_eq!(
        find_first_in_direction(1, 1, -1, 1, &mix_map),
        Some(EMPTY_SEAT_CHARACTER)
    );
    assert_eq!(
        find_first_in_direction(1, 1, 0, 1, &mix_map),
        Some(EMPTY_SEAT_CHARACTER)
    );
    assert_eq!(
        find_first_in_direction(1, 1, 1, 1, &mix_map),
        Some(OCCUPIED_SEAT_CHARACTER)
    );

    assert_eq!(
        find_first_in_direction(3, 4, -1, -1, &discover_all_occupied_map),
        Some(OCCUPIED_SEAT_CHARACTER)
    );
    assert_eq!(
        find_first_in_direction(3, 4, 0, -1, &discover_all_occupied_map),
        Some(OCCUPIED_SEAT_CHARACTER)
    );
    assert_eq!(
        find_first_in_direction(3, 4, 1, -1, &discover_all_occupied_map),
        Some(OCCUPIED_SEAT_CHARACTER)
    );
    assert_eq!(
        find_first_in_direction(3, 4, -1, 0, &discover_all_occupied_map),
        Some(OCCUPIED_SEAT_CHARACTER)
    );
    assert_eq!(
        find_first_in_direction(3, 4, 0, 0, &discover_all_occupied_map),
        None
    );
    assert_eq!(
        find_first_in_direction(3, 4, 1, 0, &discover_all_occupied_map),
        Some(OCCUPIED_SEAT_CHARACTER)
    );
    assert_eq!(
        find_first_in_direction(3, 4, -1, 1, &discover_all_occupied_map),
        Some(OCCUPIED_SEAT_CHARACTER)
    );
    assert_eq!(
        find_first_in_direction(3, 4, 0, 1, &discover_all_occupied_map),
        Some(OCCUPIED_SEAT_CHARACTER)
    );
    assert_eq!(
        find_first_in_direction(3, 4, 1, 1, &discover_all_occupied_map),
        Some(OCCUPIED_SEAT_CHARACTER)
    );
}

#[test]
fn test_update_seats() {
    let empty_seats_map = vec![
        vec!['L', 'L', 'L'],
        vec!['L', 'L', 'L'],
        vec!['L', 'L', 'L'],
    ];
    let occupied_seats_map = vec![
        vec!['#', '#', '#'],
        vec!['#', '#', '#'],
        vec!['#', '#', '#'],
    ];
    let final_map = vec![
        vec!['#', 'L', '#'],
        vec!['L', 'L', 'L'],
        vec!['#', 'L', '#'],
    ];
    let all_indices = vec![
        (0, 0),
        (1, 0),
        (2, 0),
        (0, 1),
        (1, 1),
        (2, 1),
        (0, 2),
        (1, 2),
        (2, 2),
    ];
    let mut test_map = empty_seats_map.clone();

    update_seats(&all_indices, &mut test_map);
    assert_eq!(test_map, occupied_seats_map);
    update_seats(&all_indices, &mut test_map);
    assert_eq!(test_map, empty_seats_map);

    test_map = occupied_seats_map.clone();
    let indices = vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
    update_seats(&indices, &mut test_map);
    assert_eq!(test_map, final_map);
}

#[test]
fn test_find_and_update() {
    let mut test_map = vec![
        vec!['L', 'L', 'L'],
        vec!['L', 'L', 'L'],
        vec!['L', 'L', 'L'],
    ];

    let column_len = test_map.len();
    let row_len = test_map[0].len();

    let mut indices_to_update: Vec<(usize, usize)> = Vec::new();
    for y in 0..column_len {
        for x in 0..row_len {
            if should_convert(x, y, &test_map) {
                indices_to_update.push((x, y));
            }
        }
    }

    update_seats(&indices_to_update, &mut test_map);
    assert_eq!(
        test_map,
        vec![
            vec!['#', '#', '#'],
            vec!['#', '#', '#'],
            vec!['#', '#', '#']
        ]
    );

    let mut new_indices_to_update: Vec<(usize, usize)> = Vec::new();
    for (x, y) in indices_to_update {
        if should_convert(x, y, &test_map) {
            new_indices_to_update.push((x, y));
        }
    }
    update_seats(&new_indices_to_update, &mut test_map);
    assert_eq!(
        test_map,
        vec![
            vec!['#', 'L', '#'],
            vec!['L', 'L', 'L'],
            vec!['#', 'L', '#']
        ]
    );
}

#[test]
fn test_find_and_update_part_2() {
    let test_map = vec![
        vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
        vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
        vec!['L', '.', 'L', '.', 'L', '.', '.', 'L', '.', '.'],
        vec!['L', 'L', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
        vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
        vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
        vec!['.', '.', 'L', '.', 'L', '.', '.', '.', '.', '.'],
        vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L'],
        vec!['L', '.', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L'],
        vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
    ];

    let result_map = find_and_update_part_2(&test_map);
    let result_count = count_seats_of_type(OCCUPIED_SEAT_CHARACTER, &result_map);
    assert_eq!(result_count, 26);
}
