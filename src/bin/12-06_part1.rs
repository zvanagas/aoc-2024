use std::fs::read_to_string;

fn main() {
    let file_content = read_to_string("./inputs/6.txt").expect("Error reading file");
    let mut grid: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (mut pos_x, mut pos_y) = get_guard_position(&grid);
    let mut is_in_grid = true;
    let mut guard_direction = "up";
    grid[pos_x][pos_y] = 'X';
    let mut result = 1;

    while is_in_grid {
        if guard_direction == "up" {
            if pos_x == 0 {
                is_in_grid = false;
                continue;
            }

            match grid[pos_x - 1][pos_y] {
                '.' => {
                    grid[pos_x - 1][pos_y] = 'X';
                    pos_x -= 1;
                    result += 1;
                }
                'X' => {
                    pos_x -= 1;
                }
                '#' => {
                    guard_direction = "right";
                }
                _ => {
                    is_in_grid = false;
                    println!("Invalid character found in grid: up");
                }
            }
        } else if guard_direction == "right" {
            if pos_y == grid[0].len() - 1 {
                is_in_grid = false;
                continue;
            }

            match grid[pos_x][pos_y + 1] {
                '.' => {
                    grid[pos_x][pos_y + 1] = 'X';
                    pos_y += 1;
                    result += 1;
                }
                'X' => {
                    pos_y += 1;
                }
                '#' => {
                    guard_direction = "down";
                }
                _ => {
                    is_in_grid = false;
                    println!("Invalid character found in grid: right");
                }
            }
        } else if guard_direction == "down" {
            if pos_x == grid.len() - 1 {
                is_in_grid = false;
                continue;
            }

            match grid[pos_x + 1][pos_y] {
                '.' => {
                    grid[pos_x + 1][pos_y] = 'X';
                    pos_x += 1;
                    result += 1;
                }
                'X' => {
                    pos_x += 1;
                }
                '#' => {
                    guard_direction = "left";
                }
                _ => {
                    is_in_grid = false;
                    println!("Invalid character found in grid: down");
                }
            }
        } else if guard_direction == "left" {
            if pos_y == 0 {
                is_in_grid = false;
                continue;
            }

            match grid[pos_x][pos_y - 1] {
                '.' => {
                    grid[pos_x][pos_y - 1] = 'X';
                    pos_y -= 1;
                    result += 1;
                }
                'X' => {
                    pos_y -= 1;
                }
                '#' => {
                    guard_direction = "up";
                }
                _ => {
                    is_in_grid = false;
                    println!("Invalid character found in grid: left");
                }
            }
        }
    }
    println!("{}", result);
}

fn get_guard_position(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut is_found = false;
    let mut guard_position = (0, 0);

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '^' {
                guard_position = (i, j);
                is_found = true;
                break;
            }
        }

        if is_found {
            break;
        }
    }

    guard_position
}
