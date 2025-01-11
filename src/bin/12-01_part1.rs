use std::fs::read_to_string;

fn main() {
    let file_content = read_to_string("./inputs/1.txt").expect("Error reading file");

    let mut left_map = Vec::new();
    let mut right_map = Vec::new();

    for line in file_content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
            left_map.push(left);
            right_map.push(right);
        } else {
            eprintln!("Failed to parse numbers in line: {}", line);
        }
    }

    left_map.sort();
    right_map.sort();

    let length = left_map.len();
    let mut result = 0;

    for i in 0..length {
        let left = left_map[i];
        let right = right_map[i];
        let difference = (left - right).abs();
        result += difference;
    }

    println!("Result: {}", result);
}
