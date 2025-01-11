use std::fs::read_to_string;

fn main() {
    let file_content = read_to_string("./inputs/4.txt").expect("Error reading file");
    let grid: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let word = "XMAS";
    let directions = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let rows = grid.len();
    let cols = grid[0].len();

    let mut result = 0;

    for x in 0..rows {
        for y in 0..cols {
            for (dx, dy) in &directions {
                if check_direction(&grid, x, y, *dx, *dy, word) {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);
}

fn check_direction(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    word: &str,
) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word.len();

    for i in 0..word_len {
        let nx = x as isize + i as isize * dx;
        let ny = y as isize + i as isize * dy;

        if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize {
            return false;
        }

        if grid[nx as usize][ny as usize] != word.chars().nth(i).unwrap() {
            return false;
        }
    }

    true
}
