use std::fs::read_to_string;

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

        let mut trend = None;
        let valid = numbers.windows(2).all(|pair| {
            let (previous, current) = (pair[0], pair[1]);
            let current_trend = if current > previous {
                Some(true)
            } else if current < previous {
                Some(false)
            } else {
                None
            };

            if let Some(t) = trend {
                if Some(t) != current_trend {
                    return false;
                }
            }

            if trend.is_none() {
                trend = current_trend;
            }

            let diff = (current - previous).abs();
            diff >= 1 && diff <= 3
        });

        if valid && trend.is_some() {
            result += 1;
        }
    }

    println!("{}", result);
}
