use std::env;
use std::fs::File;
use std::io::Read;

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn split_input(string: &String) -> Vec<(char, i32)> {
    let lines = string.split("\n");
    let mut result: Vec<(char, i32)> = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (first, rest) = line.split_at(1);
        if let Ok(rest) = rest.parse() {
            if let Some(character) = first.chars().next() {
                result.push((character, rest));
            }
        }
    }
    result
}

const NORTH: char = 'N';
const SOUTH: char = 'S';
const EAST: char = 'E';
const WEST: char = 'W';
const LEFT: char = 'L';
const RIGHT: char = 'R';
const FORWARD: char = 'F';

#[derive(Debug, Clone, PartialEq)]
struct Ship {
    latitude: i32,
    longitude: i32,
    facing: char,
}

fn rotate_ship(direction: char, amount: i32, ship: &mut Ship) {
    const FACINGS: [char; 4] = [NORTH, EAST, SOUTH, WEST];
    if (direction != LEFT && direction != RIGHT) || amount % 360 == 0 {
        return;
    }

    let amount = if direction == LEFT {
        amount / -90
    } else {
        amount / 90
    };
    if let Some(start_index) = FACINGS.iter().position(|&facing| facing == ship.facing) {
        let new_index = (start_index as i32 + amount + FACINGS.len() as i32) % 4;
        if let Some(new_facing) = FACINGS.iter().nth(new_index as usize) {
            ship.facing = *new_facing;
        }
    }
}

fn move_ship(direction: char, amount: i32, ship: &mut Ship) {
    if amount == 0 {
        return;
    }

    let mut direction = direction;
    if direction == FORWARD {
        direction = ship.facing;
    }

    if direction == NORTH {
        ship.latitude += amount;
    } else if direction == SOUTH {
        ship.latitude -= amount
    } else if direction == EAST {
        ship.longitude += amount;
    } else if direction == WEST {
        ship.longitude -= amount;
    } else if direction == LEFT || direction == RIGHT {
        rotate_ship(direction, amount, ship);
    }
}

fn follow_instructions(instructions: &Vec<(char, i32)>) -> Ship {
    let mut ship = Ship {
        latitude: 0,
        longitude: 0,
        facing: EAST,
    };
    for (direction, amount) in instructions {
        move_ship(*direction, *amount, &mut ship);
    }
    ship
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = read_file(path).expect("Error reading file.");
    let input = split_input(&input);
    let part_1 = follow_instructions(&input);
    let part_1 = part_1.latitude.wrapping_abs() + part_1.longitude.wrapping_abs();
    println!("part_1: {:?}", part_1);
}

#[test]
fn test_split_input() {
    assert_eq!(split_input(&String::from("")), vec![]);
    assert_eq!(
        split_input(&String::from("R1\nF50")),
        vec![('R', 1), ('F', 50)]
    );
    assert_eq!(split_input(&String::from("R1")), vec![('R', 1)]);
}

#[test]
fn test_move_ship() {
    let mut test_ship: Ship = Ship {
        latitude: 0,
        longitude: 0,
        facing: NORTH,
    };
    let initial_ship: Ship = Ship {
        latitude: 0,
        longitude: 0,
        facing: NORTH,
    };

    move_ship(NORTH, 0, &mut test_ship);
    assert_eq!(test_ship, initial_ship);

    test_ship = initial_ship.clone();
    move_ship(NORTH, 1, &mut test_ship);
    assert_eq!(
        test_ship,
        Ship {
            latitude: 1,
            longitude: 0,
            facing: NORTH
        }
    );

    test_ship = initial_ship.clone();
    move_ship(SOUTH, 0, &mut test_ship);
    assert_eq!(test_ship, initial_ship);

    test_ship = initial_ship.clone();
    move_ship(SOUTH, 1, &mut test_ship);
    assert_eq!(
        test_ship,
        Ship {
            latitude: -1,
            longitude: 0,
            facing: NORTH
        }
    );

    test_ship = initial_ship.clone();
    move_ship(EAST, 0, &mut test_ship);
    assert_eq!(test_ship, initial_ship);

    test_ship = initial_ship.clone();
    move_ship(EAST, 1, &mut test_ship);
    assert_eq!(
        test_ship,
        Ship {
            latitude: 0,
            longitude: 1,
            facing: NORTH
        }
    );

    test_ship = initial_ship.clone();
    move_ship(WEST, 0, &mut test_ship);
    assert_eq!(test_ship, initial_ship);

    test_ship = initial_ship.clone();
    move_ship(WEST, 1, &mut test_ship);
    assert_eq!(
        test_ship,
        Ship {
            latitude: 0,
            longitude: -1,
            facing: NORTH
        }
    );

    test_ship = initial_ship.clone();
    move_ship(LEFT, 0, &mut test_ship);
    assert_eq!(test_ship, initial_ship);

    test_ship = initial_ship.clone();
    move_ship(LEFT, 90, &mut test_ship);
    assert_eq!(
        test_ship,
        Ship {
            latitude: 0,
            longitude: 0,
            facing: WEST
        }
    );

    test_ship = initial_ship.clone();
    move_ship(LEFT, 180, &mut test_ship);
    assert_eq!(
        test_ship,
        Ship {
            latitude: 0,
            longitude: 0,
            facing: SOUTH
        }
    );

    test_ship = initial_ship.clone();
    move_ship(LEFT, 270, &mut test_ship);
    assert_eq!(
        test_ship,
        Ship {
            latitude: 0,
            longitude: 0,
            facing: EAST
        }
    );

    test_ship = initial_ship.clone();
    move_ship(LEFT, 360, &mut test_ship);
    assert_eq!(test_ship, initial_ship);

    test_ship = initial_ship.clone();
    move_ship(RIGHT, 0, &mut test_ship);
    assert_eq!(test_ship, initial_ship);

    test_ship = initial_ship.clone();
    move_ship(RIGHT, 90, &mut test_ship);
    assert_eq!(
        test_ship,
        Ship {
            latitude: 0,
            longitude: 0,
            facing: EAST
        }
    );

    test_ship = initial_ship.clone();
    move_ship(RIGHT, 180, &mut test_ship);
    assert_eq!(
        test_ship,
        Ship {
            latitude: 0,
            longitude: 0,
            facing: SOUTH
        }
    );

    test_ship = initial_ship.clone();
    move_ship(RIGHT, 270, &mut test_ship);
    assert_eq!(
        test_ship,
        Ship {
            latitude: 0,
            longitude: 0,
            facing: WEST
        }
    );

    test_ship = initial_ship.clone();
    move_ship(RIGHT, 360, &mut test_ship);
    assert_eq!(test_ship, initial_ship);

    test_ship = initial_ship.clone();
    move_ship(FORWARD, 0, &mut test_ship);
    assert_eq!(test_ship, initial_ship);

    test_ship = initial_ship.clone();
    move_ship(FORWARD, 1, &mut test_ship);
    assert_eq!(
        test_ship,
        Ship {
            latitude: 1,
            longitude: 0,
            facing: NORTH
        }
    );
}
