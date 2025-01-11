use std::fs::read_to_string;

fn is_valid_sequence(seq: &[i32]) -> bool {
    let mut trend = None;

    for i in 1..seq.len() {
        let diff = (seq[i] - seq[i - 1]).abs();

        let current_trend = if seq[i] > seq[i - 1] {
            Some(true)
        } else if seq[i] < seq[i - 1] {
            Some(false)
        } else {
            None
        };

        if diff < 1 || diff > 3 {
            return false;
        }

        if let Some(t) = trend {
            if Some(t) != current_trend {
                return false;
            }
        }

        if trend.is_none() {
            trend = current_trend;
        }
    }
    true
}

fn main() {
    let file_content = read_to_string("./inputs/2.txt").expect("Error reading file");

    let mut result = 0;

    for line in file_content.lines() {
        let numbers: Vec<i32> = match line
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(nums) => nums,
            Err(_) => continue,
        };

        if numbers.len() < 2 {
            continue;
        }

        if is_valid_sequence(&numbers) {
            result += 1;
            continue;
        }

        let mut is_safe_with_one_removal = false;
        for i in 0..numbers.len() {
            let mut new_sequence = numbers.clone();
            new_sequence.remove(i);
            if is_valid_sequence(&new_sequence) {
                is_safe_with_one_removal = true;
                break;
            }
        }

        if is_safe_with_one_removal {
            result += 1;
        }
    }

    println!("{}", result);
}
