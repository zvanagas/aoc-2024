use std::{collections::BTreeMap, fs::read_to_string};

fn main() {
    let file_content = match read_to_string("./inputs/1.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut left_map = Vec::new();
    let mut right_map = BTreeMap::new();

    for line in file_content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
            left_map.push(left);
            *right_map.entry(right).or_insert(0) += 1;
        } else {
            eprintln!("Failed to parse numbers in line: {}", line);
        }
    }

    left_map.sort();

    let mut result = 0;

    for left in left_map {
        let occurrences = right_map.get(&left).cloned().unwrap_or(0);
        let total = left * occurrences;
        result += total;
    }

    println!("Result: {}", result);
}
