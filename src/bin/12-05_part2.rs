use std::{cmp::Ordering, collections::HashSet, fs::read_to_string};

fn main() {
    let file_content = read_to_string("./inputs/5.txt").expect("Error reading file");

    let sections = file_content.split("\n\n").collect::<Vec<&str>>();

    let rules: HashSet<(i32, i32)> = sections[0]
        .lines()
        .map(|line| (line[0..2].parse().unwrap(), line[3..].parse().unwrap()))
        .collect();
    let updates = sections[1].lines().collect::<Vec<&str>>();

    let mut result = 0;

    for update in updates {
        let mut items = update
            .split(",")
            .map(|num| num.parse::<i32>().expect("Failed to parse number"))
            .collect::<Vec<i32>>();
        if !items
            .is_sorted_by(|&a, &b| !rules.contains(&(b.try_into().unwrap(), a.try_into().unwrap())))
        {
            items.sort_by(|&a, &b| {
                if rules.contains(&(a, b)) {
                    Ordering::Less
                } else if rules.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            result += items[items.len() / 2];
        }
    }

    println!("{}", result);
}
