use std::env;
use std::collections::HashMap;

mod file_reader {
    use std::fs::File;
    use std::io::Read;
    pub fn read_file(file_path: &str) -> std::io::Result<String> {
        let mut file = File::open(file_path)?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        Ok(string)
    }

    pub fn split_input(string: &String) -> Vec<&str> {
        let strings: Vec<&str> = string
            .split("\n")
            .collect();
        strings
    }
}

const NO_BAGS_EXPRESSION: &str = "no other bags";
const BAGS_CONTAIN_EXPRESSION: &str = " bags contain ";
const BAGS_ENDING_EXPRESSION: &str = " bags";
const BAG_ENDING_EXPRESSION: &str = " bag";

#[derive(Debug)]
struct Bag<'a> {
    bag_type: &'a str,
    amount: u32
}

fn parse_line(line: &str) -> (&str, Vec<Bag>) {
    let outer_bag_index = line.find(BAGS_CONTAIN_EXPRESSION).unwrap();
    let (outer_bag, line) = line.split_at(outer_bag_index);
    let mut inner_bags: Vec<Bag> = Vec::new();
    if !line.contains(NO_BAGS_EXPRESSION) {
        let (_, line) = line.split_at(BAGS_CONTAIN_EXPRESSION.len());
        let bags: Vec<&str> = line
            .split(", ")
            .map(|string| string
                .trim_end_matches(".")
                .trim_end_matches(BAG_ENDING_EXPRESSION)
                .trim_end_matches(BAGS_ENDING_EXPRESSION))
            .collect();

        for bag in bags {
            if let Some(space_index) = bag.find(" ") {
                let (amount, bag_type) = bag.split_at(space_index);
                inner_bags.push(Bag{bag_type: bag_type.trim(), amount: amount.parse().unwrap()});
            }
        }
    }
    (outer_bag, inner_bags)
}

fn can_bag_contain(outer_bag_type: &str, target_bag_type: &str, bags_map: &HashMap<&str, Vec<Bag>>) -> bool {
    let bags = &bags_map[outer_bag_type];
    for bag in bags {
        if bag.bag_type == target_bag_type {
            return true
        } else if can_bag_contain(bag.bag_type, target_bag_type, bags_map) {
            return true
        }
    }
    false
}

fn count_bags_that_can_contain(target_bag: &str, bags_map: &HashMap<&str, Vec<Bag>>) -> u32 {
    let mut count: u32 = 0;

    for (outer_bag, _) in bags_map {
        if can_bag_contain(outer_bag, target_bag, bags_map) {
            count += 1;
        }
    }

    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let input = file_reader::read_file(path).unwrap();
    let input = file_reader::split_input(&input);
    let mut iter = input.iter();
    let mut bags: HashMap<&str, Vec<Bag>> = HashMap::new();
    while let Some(line) = &iter.next() {
        let (outer_bag, inner_bags) = parse_line(&line);
        bags.insert(outer_bag, inner_bags);
    }

    let part_1 = count_bags_that_can_contain("shiny gold", &bags);
    println!("{:?}", part_1);
}

#[test]
fn test_can_bag_contain() {
    let mut test_map: HashMap<&str, Vec<Bag>> = HashMap::new();
    test_map.insert("light red", vec![Bag{bag_type: "bright white", amount: 1}, Bag{bag_type: "muted yellow", amount: 2}]);
    test_map.insert("dark orange", vec![Bag{bag_type: "bright white", amount: 3}, Bag{bag_type: "muted yellow", amount: 4}]);
    test_map.insert("bright white", vec![Bag{bag_type: "shiny gold", amount: 1}]);
    test_map.insert("muted yellow", vec![Bag{bag_type: "shiny gold", amount: 2}, Bag{bag_type: "faded blue", amount: 9}]);
    test_map.insert("shiny gold", vec![Bag{bag_type: "dark olive", amount: 1}, Bag{bag_type: "vibrant plum", amount: 2}]);
    test_map.insert("dark olive", vec![Bag{bag_type: "faded blue", amount: 3}, Bag{bag_type: "dotted black", amount: 4}]);
    test_map.insert("vibrant plum", vec![Bag{bag_type: "faded blue", amount: 5}, Bag{bag_type: "dotted black", amount: 6}]);
    test_map.insert("faded blue", vec![]);
    test_map.insert("dotted black", vec![]);

    assert_eq!(can_bag_contain("vibrant plum", "faded blue", &test_map), true);
    assert_eq!(can_bag_contain("vibrant plum", "light red", &test_map), false);
    assert_eq!(can_bag_contain("light red", "faded blue", &test_map), true);

    assert_eq!(can_bag_contain("bright white", "shiny gold", &test_map), true);
    assert_eq!(can_bag_contain("muted yellow", "shiny gold", &test_map), true);
    assert_eq!(can_bag_contain("dark orange", "shiny gold", &test_map), true);
    assert_eq!(can_bag_contain("light red", "shiny gold", &test_map), true);
}
