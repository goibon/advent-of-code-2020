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
    let lines = string.split('\n');
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

#[derive(Debug, Clone, PartialEq)]
struct Waypoint {
    latitude: i32,
    longitude: i32,
}

fn change_cardinal_direction(initial_direction: char, direction: char, amount: i32) -> char {
    assert!(direction == LEFT || direction == RIGHT);

    const CARDINAL_DIRECTIONS: [char; 4] = [NORTH, EAST, SOUTH, WEST];
    let amount = if direction == LEFT {
        amount / -90
    } else {
        amount / 90
    };
    if let Some(start_index) = CARDINAL_DIRECTIONS.iter().position(|&facing| facing == initial_direction) {
        let new_index = (start_index as i32 + amount + CARDINAL_DIRECTIONS.len() as i32) % 4;
        if let Some(new_facing) = CARDINAL_DIRECTIONS.iter().nth(new_index as usize) {
            return *new_facing;
        }
    }
    initial_direction
}

fn rotate_ship(direction: char, amount: i32, ship: &mut Ship) {
    if (direction != LEFT && direction != RIGHT) || amount % 360 == 0 {
        return;
    }

    ship.facing = change_cardinal_direction(ship.facing, direction, amount);
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

fn move_ship_to_waypoint(amount: i32, ship: &mut Ship, waypoint: &Waypoint) {
    if amount == 0 {
        return;
    }

    ship.latitude += waypoint.latitude * amount;
    ship.longitude += waypoint.longitude * amount;
}

fn rotate_waypoint(direction: char, amount: i32, waypoint: &mut Waypoint) {
    if (direction != LEFT && direction != RIGHT) || amount % 360 == 0 {
        return;
    }

    if amount == 180 {
        waypoint.latitude = -waypoint.latitude;
        waypoint.longitude = -waypoint.longitude;
    } else if direction == LEFT && amount == 90 || (direction == RIGHT && amount == 270) {
        let new_latitude = waypoint.longitude;
        let new_longitude = -waypoint.latitude;
        waypoint.latitude = new_latitude;
        waypoint.longitude = new_longitude;
    } else if direction == RIGHT || (direction == LEFT && amount == 270) {
        let new_latitude = -waypoint.longitude;
        let new_longitude = waypoint.latitude;
        waypoint.latitude = new_latitude;
        waypoint.longitude = new_longitude;
    }
}

fn move_waypoint(direction: char, amount: i32, waypoint: &mut Waypoint) {
    if amount == 0 || direction == FORWARD {
        return;
    }

    if direction == NORTH {
        waypoint.latitude += amount;
    } else if direction == SOUTH {
        waypoint.latitude -= amount
    } else if direction == EAST {
        waypoint.longitude += amount;
    } else if direction == WEST {
        waypoint.longitude -= amount;
    } else if direction == LEFT || direction == RIGHT {
        rotate_waypoint(direction, amount, waypoint);
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

fn follow_instructions_part_2(instructions: &Vec<(char, i32)>) -> Ship {
    let mut ship = Ship {
        latitude: 0,
        longitude: 0,
        facing: EAST
    };
    let mut waypoint = Waypoint {
        latitude: 1,
        longitude: 10
    };

    for (direction, amount) in instructions {
        if *direction == FORWARD {
            move_ship_to_waypoint(*amount, &mut ship, &waypoint);
        } else {
            move_waypoint(*direction, *amount, &mut waypoint);
        }
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

    let part_2 = follow_instructions_part_2(&input);
    println!("part_2: {:?}", part_2);
    let part_2 = part_2.latitude.wrapping_abs() + part_2.longitude.wrapping_abs();
    println!("part_2: {:?}", part_2);
}

#[test]
fn test_follow_instructions_part_2() {
    let instructions = vec![
        (FORWARD, 10),
        (NORTH, 3),
        (FORWARD, 7),
        (RIGHT, 90),
        (FORWARD, 11),
    ];
    let test_ship = follow_instructions_part_2(&instructions);
    assert_eq!(test_ship, Ship {
        latitude: -72,
        longitude: 214,
        facing: EAST
    });
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
fn test_change_cardinal_direction() {
    assert_eq!(change_cardinal_direction(NORTH, LEFT, 0), NORTH);
    assert_eq!(change_cardinal_direction(NORTH, LEFT, 720), NORTH);
    assert_eq!(change_cardinal_direction(NORTH, LEFT, 90), WEST);
    assert_eq!(change_cardinal_direction(NORTH, LEFT, 180), SOUTH);
    assert_eq!(change_cardinal_direction(NORTH, LEFT, 270), EAST);
    assert_eq!(change_cardinal_direction(NORTH, RIGHT, 0), NORTH);
    assert_eq!(change_cardinal_direction(NORTH, RIGHT, 720), NORTH);
    assert_eq!(change_cardinal_direction(NORTH, RIGHT, 90), EAST);
    assert_eq!(change_cardinal_direction(NORTH, RIGHT, 180), SOUTH);
    assert_eq!(change_cardinal_direction(NORTH, RIGHT, 270), WEST);

    assert_eq!(change_cardinal_direction(EAST, LEFT, 0), EAST);
    assert_eq!(change_cardinal_direction(EAST, LEFT, 720), EAST);
    assert_eq!(change_cardinal_direction(EAST, LEFT, 90), NORTH);
    assert_eq!(change_cardinal_direction(EAST, LEFT, 180), WEST);
    assert_eq!(change_cardinal_direction(EAST, LEFT, 270), SOUTH);
    assert_eq!(change_cardinal_direction(EAST, RIGHT, 0), EAST);
    assert_eq!(change_cardinal_direction(EAST, RIGHT, 720), EAST);
    assert_eq!(change_cardinal_direction(EAST, RIGHT, 90), SOUTH);
    assert_eq!(change_cardinal_direction(EAST, RIGHT, 180), WEST);
    assert_eq!(change_cardinal_direction(EAST, RIGHT, 270), NORTH);
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

#[test]
fn test_move_ship_to_waypoint() {
    let initial_ship: Ship = Ship {
        latitude: 0,
        longitude: 0,
        facing: EAST,
    };

    let mut test_ship = initial_ship.clone();
    move_ship_to_waypoint(0, &mut test_ship, &Waypoint {
        latitude: 10,
        longitude: 1,
    });
    assert_eq!(test_ship, initial_ship);

    move_ship_to_waypoint(10, &mut test_ship, &Waypoint {
        latitude: 10,
        longitude: 1,
    });
    assert_eq!(test_ship, Ship { latitude: 100, longitude: 10, facing: EAST});

    let mut test_ship = initial_ship.clone();
    move_ship_to_waypoint(10, &mut test_ship, &Waypoint {
        latitude: -10,
        longitude: 1,
    });
    assert_eq!(test_ship, Ship { latitude: -100, longitude: 10, facing: EAST});

    let mut test_ship = initial_ship.clone();
    move_ship_to_waypoint(10, &mut test_ship, &Waypoint {
        latitude: 1,
        longitude: -10,
    });
    assert_eq!(test_ship, Ship { latitude: 10, longitude: -100, facing: EAST});

    let mut test_ship = initial_ship.clone();
    move_ship_to_waypoint(10, &mut test_ship, &Waypoint {
        latitude: -1,
        longitude: -10,
    });
    assert_eq!(test_ship, Ship { latitude: -10, longitude: -100, facing: EAST});
}

#[test]
fn test_move_waypoint() {
    let initial_waypoint = Waypoint {
        latitude: 10,
        longitude: 1,
    };

    let mut test_waypoint = initial_waypoint.clone();
    move_waypoint(NORTH, 0, &mut test_waypoint);
    assert_eq!(test_waypoint, initial_waypoint);
    move_waypoint(SOUTH, 0, &mut test_waypoint);
    assert_eq!(test_waypoint, initial_waypoint);
    move_waypoint(EAST, 0, &mut test_waypoint);
    assert_eq!(test_waypoint, initial_waypoint);
    move_waypoint(WEST, 0, &mut test_waypoint);
    assert_eq!(test_waypoint, initial_waypoint);
    move_waypoint(LEFT, 0, &mut test_waypoint);
    assert_eq!(test_waypoint, initial_waypoint);
    move_waypoint(RIGHT, 0, &mut test_waypoint);
    assert_eq!(test_waypoint, initial_waypoint);

    let mut test_waypoint = initial_waypoint.clone();
    move_waypoint(NORTH, 10, &mut test_waypoint);
    assert_eq!(test_waypoint, Waypoint { latitude: 20, longitude: 1 });

    let mut test_waypoint = initial_waypoint.clone();
    move_waypoint(SOUTH, 10, &mut test_waypoint);
    assert_eq!(test_waypoint, Waypoint { latitude: 0, longitude: 1 });

    let mut test_waypoint = initial_waypoint.clone();
    move_waypoint(EAST, 10, &mut test_waypoint);
    assert_eq!(test_waypoint, Waypoint { latitude: 10, longitude: 11 });

    let mut test_waypoint = initial_waypoint.clone();
    move_waypoint(WEST, 10, &mut test_waypoint);
    assert_eq!(test_waypoint, Waypoint { latitude: 10, longitude: -9 });

    let mut test_waypoint = initial_waypoint.clone();
    move_waypoint(LEFT, 0, &mut test_waypoint);
    assert_eq!(test_waypoint, initial_waypoint);
    let mut test_waypoint = initial_waypoint.clone();
    move_waypoint(LEFT, 360, &mut test_waypoint);
    assert_eq!(test_waypoint, initial_waypoint);

    let mut test_waypoint = initial_waypoint.clone();
    move_waypoint(LEFT, 90, &mut test_waypoint);
    assert_eq!(test_waypoint, Waypoint { latitude: 1, longitude: -10 });

    let mut test_waypoint = initial_waypoint.clone();
    move_waypoint(LEFT, 180, &mut test_waypoint);
    assert_eq!(test_waypoint, Waypoint { latitude: -10, longitude: -1  });

    let mut test_waypoint = initial_waypoint.clone();
    move_waypoint(LEFT, 270, &mut test_waypoint);
    assert_eq!(test_waypoint, Waypoint { latitude: -1, longitude: 10 });

    let mut test_waypoint = initial_waypoint.clone();
    move_waypoint(RIGHT, 0, &mut test_waypoint);
    assert_eq!(test_waypoint, initial_waypoint);
    let mut test_waypoint = initial_waypoint.clone();
    move_waypoint(RIGHT, 360, &mut test_waypoint);
    assert_eq!(test_waypoint, initial_waypoint);

    let mut test_waypoint = initial_waypoint.clone();
    move_waypoint(RIGHT, 90, &mut test_waypoint);
    assert_eq!(test_waypoint, Waypoint { latitude: -1, longitude: 10 });

    let mut test_waypoint = initial_waypoint.clone();
    move_waypoint(RIGHT, 180, &mut test_waypoint);
    assert_eq!(test_waypoint, Waypoint { latitude: -10, longitude: -1  });

    let mut test_waypoint = initial_waypoint.clone();
    move_waypoint(RIGHT, 270, &mut test_waypoint);
    assert_eq!(test_waypoint, Waypoint { latitude: 1, longitude: -10 });
}
