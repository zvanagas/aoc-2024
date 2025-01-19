use std::fs::read_to_string;

fn main() {
    let file_content = read_to_string("./inputs/7.txt").expect("Error reading file");

    let mut equations: Vec<(usize, Vec<usize>)> = Vec::new();

    for line in file_content.lines() {
        let (result, numbers) = line.split_once(':').unwrap();
        let result = result.parse().unwrap();
        let numbers = numbers
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        equations.push((result, numbers));
    }
    let mut result = 0;
    for (target, numbers) in equations {
        let (start, numbers) = numbers.split_first().unwrap();
        if can_reach(*start, target, numbers) {
            result += target;
        }
    }

    println!("{}", result);
}

fn can_reach(start: usize, target: usize, numbers: &[usize]) -> bool {
    if numbers.len() == 0 {
        return start == target;
    }
    if start > target {
        return false;
    }
    let (first, rest) = numbers.split_first().unwrap();
    can_reach(start * first, target, rest) || can_reach(start + first, target, rest)
}
