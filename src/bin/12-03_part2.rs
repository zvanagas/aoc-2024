use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let file_content = read_to_string("./inputs/3-2.txt").expect("Error reading file");

    let mut result = 0;
    let mut is_enabled = true;

    let regex = Regex::new(r"do\(\)|don\'t\(\)|mul\((\d+),(\d+)\)").unwrap();

    for capture in regex.captures_iter(&file_content) {
        if &capture[0] == "do()" {
            is_enabled = true;
            continue;
        } else if &capture[0] == "don't()" {
            is_enabled = false;
            continue;
        }

        if is_enabled {
            let num1: i32 = capture[1].parse().unwrap();
            let num2: i32 = capture[2].parse().unwrap();

            result += num1 * num2;
        }
    }

    print!("{}", result);
}
