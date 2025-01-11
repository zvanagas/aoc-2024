use std::fs::read_to_string;

fn main() {
    let file_content = read_to_string("./inputs/4.txt").expect("Error reading file");
    let grid: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut result = 0;

    for x in 1..rows - 1 {
        for y in 1..cols - 1 {
            if grid[x][y] == 'A' && is_x_pattern(&grid, x, y) {
                result += 1;
            }
        }
    }

    println!("{}", result);
}

fn is_x_pattern(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || x + 1 >= grid.len() || y + 1 >= grid[0].len() {
        return false;
    }

    let mas_mas = grid[x - 1][y - 1] == 'M'
        && grid[x][y] == 'A'
        && grid[x + 1][y + 1] == 'S'
        && grid[x - 1][y + 1] == 'M'
        && grid[x + 1][y - 1] == 'S';

    let mas_sam = grid[x - 1][y - 1] == 'M'
        && grid[x][y] == 'A'
        && grid[x + 1][y + 1] == 'S'
        && grid[x - 1][y + 1] == 'S'
        && grid[x + 1][y - 1] == 'M';

    let sam_mas = grid[x - 1][y - 1] == 'S'
        && grid[x][y] == 'A'
        && grid[x + 1][y + 1] == 'M'
        && grid[x - 1][y + 1] == 'M'
        && grid[x + 1][y - 1] == 'S';

    let sam_sam = grid[x - 1][y + 1] == 'S'
        && grid[x][y] == 'A'
        && grid[x + 1][y - 1] == 'M'
        && grid[x - 1][y - 1] == 'S'
        && grid[x + 1][y + 1] == 'M';

    mas_mas || mas_sam || sam_mas || sam_sam
}
