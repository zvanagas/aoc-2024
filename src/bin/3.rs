use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let file_content = match read_to_string("./inputs/3.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut result = 0;

    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for capture in regex.captures_iter(&file_content) {
        let num1: i32 = capture[1].parse().unwrap();
        let num2: i32 = capture[2].parse().unwrap();

        result += num1 * num2;
    }

    print!("{}", result);
}
