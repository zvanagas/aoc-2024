use std::{collections::BTreeMap, fs::read_to_string};

fn main() {
    let file_content = read_to_string("./inputs/5.txt").expect("Error reading file");

    let sections = file_content.split("\n\n").collect::<Vec<&str>>();

    let rules = sections[0]
        .lines()
        .map(|line| {
            line.split("|")
                .map(|num| num.parse::<i32>().expect("Failed to parse number"))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let updates = sections[1].lines().collect::<Vec<&str>>();

    let mut result = 0;

    for update in updates {
        let (is_valid, mid) = follows_rules(update, &rules);
        if is_valid {
            result += mid;
        }
    }

    println!("{}", result);
}

fn follows_rules(update: &str, rules: &Vec<Vec<i32>>) -> (bool, i32) {
    let mut tree_map = BTreeMap::new();
    let items = update
        .split(",")
        .map(|num| num.parse::<i32>().expect("Failed to parse number"))
        .collect::<Vec<i32>>();
    for i in 0..items.len() {
        tree_map.insert(items[i], i);
    }

    for rule in rules {
        if tree_map.contains_key(&rule[0])
            && tree_map.contains_key(&rule[1])
            && tree_map.get(&rule[0]) > tree_map.get(&rule[1])
        {
            return (false, 0);
        }
    }

    (true, items[items.len() / 2])
}
