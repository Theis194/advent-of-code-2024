use crate::read_file;

pub fn result() {
    let result1 = solution1();
    println!("Result: {}", result1);
    let result2 = solution2();
    println!("Result: {}", result2);
}

fn solution1() -> i32 {
    let input = read_file("src/data/day3.txt");

    let height = input.len();
    let width = input[0].len();

    let mut grid = vec![vec![' '; width]; height];

    for i in 0..input.len() {
        let chars: Vec<char> = input[i].chars().collect();

        for j in 0..chars.len() {
            grid[i][j] = chars[j];
        }
    }

    find_xmas(&grid)
}

fn solution2() -> i32 {
    let input = read_file("src/data/day3.txt");

    let height = input.len();
    let width = input[0].len();

    let mut grid = vec![vec![' '; width]; height];

    for i in 0..input.len() {
        let chars: Vec<char> = input[i].chars().collect();

        for j in 0..chars.len() {
            grid[i][j] = chars[j];
        }
    }

    find_x_mas(&grid)
}

fn find_x_mas(grid: &[Vec<char>]) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut matches = 0;
    for row in 0..rows {
        for col in 0..cols {
            if is_x_mas(grid, row, col) {
                matches += 1;
            }
        }
    }

    matches
}

fn is_x_mas(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    if row == 0 || row == rows - 1 || col == 0 || col == cols - 1 {
        return false;
    }

    let top_left = (row - 1, col - 1);
    let bottom_right = (row + 1, col + 1);

    // Check the diagonal from top-right to bottom-left
    let top_right = (row - 1, col + 1);
    let bottom_left = (row + 1, col - 1);

    if grid[top_left.0][top_left.1] == 'M'
        && grid[row][col] == 'A'
        && grid[bottom_right.0][bottom_right.1] == 'S'
        && grid[top_right.0][top_right.1] == 'M'
        && grid[bottom_left.0][bottom_left.1] == 'S'
    {
        return true
    }

    if grid[top_left.0][top_left.1] == 'S'
        && grid[row][col] == 'A'
        && grid[bottom_right.0][bottom_right.1] == 'M'
        && grid[top_right.0][top_right.1] == 'S'
        && grid[bottom_left.0][bottom_left.1] == 'M'
    {
        return true
    }

    if grid[top_left.0][top_left.1] == 'M'
        && grid[row][col] == 'A'
        && grid[bottom_right.0][bottom_right.1] == 'S'
        && grid[top_right.0][top_right.1] == 'S'
        && grid[bottom_left.0][bottom_left.1] == 'M'
    {
        return true
    }

    if grid[top_left.0][top_left.1] == 'S'
        && grid[row][col] == 'A'
        && grid[bottom_right.0][bottom_right.1] == 'M'
        && grid[top_right.0][top_right.1] == 'M'
        && grid[bottom_left.0][bottom_left.1] == 'S'
    {
        return true
    }

    false
}

fn find_xmas(grid: &[Vec<char>]) -> i32 {
    let target = ['X', 'M', 'A', 'S'];
    let rows = grid.len();
    let cols = grid[0].len();

    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Diagonal Down-Right
        (-1, -1), // Diagonal Up-Left
        (1, -1),  // Diagonal Down-Left
        (-1, 1),  // Diagonal Up-Right
    ];

    let mut matches = 0;
    for row in 0..rows {
        for col in 0..cols {
            for &(row_delta, col_delta) in &directions {
                if let Some(sequence) =
                    find_sequence(grid, (row, col), &target, row_delta, col_delta)
                {
                    matches += 1;
                }
            }
        }
    }

    matches
}

fn find_sequence(
    grid: &[Vec<char>],
    start: (usize, usize),
    target: &[char],
    row_delta: isize,
    col_delta: isize,
) -> Option<Vec<(usize, usize)>> {
    let (mut row, mut col) = (start.0 as isize, start.1 as isize);
    let mut positions = vec![];

    for &ch in target {
        if row < 0 || col < 0 || row >= grid.len() as isize || col >= grid[0].len() as isize {
            return None; // Out of bounds
        }
        if grid[row as usize][col as usize] != ch {
            return None; // Character does not match
        }
        positions.push((row as usize, col as usize));
        row += row_delta;
        col += col_delta;
    }

    Some(positions)
}
