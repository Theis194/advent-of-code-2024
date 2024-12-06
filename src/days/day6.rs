use crate::read_file;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up((i32, i32)),
    Down((i32, i32)),
    Left((i32, i32)),
    Right((i32, i32)),
}

impl Direction {
    fn new(dir_char: &char) -> Direction {
        match dir_char {
            '^' => Direction::new_up(),
            'v' => Direction::new_down(),
            '<' => Direction::new_left(),
            '>' => Direction::new_right(),
            _ => panic!("Invalid direction character: {}", dir_char),
        }
    }

    fn new_up() -> Direction {
        Direction::Up((-1, 0))
    }

    fn new_down() -> Direction {
        Direction::Down((1, 0))
    }

    fn new_left() -> Direction {
        Direction::Left((0, -1))
    }

    fn new_right() -> Direction {
        Direction::Right((0, 1))
    }

    fn get_direction(&self) -> (i32, i32) {
        match self {
            Direction::Up((x, y)) => (*x, *y),
            Direction::Down((x, y)) => (*x, *y),
            Direction::Left((x, y)) => (*x, *y),
            Direction::Right((x, y)) => (*x, *y),
        }
    }

    fn rotate(&self) -> Direction {
        match self {
            Direction::Up(_) => Direction::new_right(),
            Direction::Down(_) => Direction::new_left(),
            Direction::Left(_) => Direction::new_up(),
            Direction::Right(_) => Direction::new_down(),
        }
    }
}

pub fn result() {
    let result1 = solution1();
    println!("Result: {}", result1);
    let result2 = solution2();
    println!("Result: {}", result2);
}

fn solution1() -> usize {
    let map = setup();
    let guard = find_guard(&map).unwrap();

    let mut visited = HashSet::new();
    visited.insert((guard.0, guard.1));

    step(&map, guard.0, guard.1, guard.2, 0, &mut visited);

    visited.len()
}

fn solution2() -> usize {
    let map = setup();
    let guard = find_guard(&map).unwrap();

    let mut valid_positions = HashSet::new();

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            // Skip if it's an existing obstacle or the guard's starting position
            if map[row][col] == '#' || (row == guard.0 && col == guard.1) {
                continue;
            }

            // Clone the map and add a temporary obstacle
            let mut temp_map = map.clone();
            temp_map[row][col] = '#';

            // Simulate the guard's movement with the new obstacle
            let mut visited = HashSet::new();
            visited.insert((guard.0, guard.1));

            let mut path = HashSet::new();
            let loop_detected = simulate_with_obstruction(&temp_map, guard.0, guard.1, guard.2.clone(), &mut visited, &mut path);

            // If a loop is detected, this position is valid
            if loop_detected {
                valid_positions.insert((row, col));
            }
        }
    }

    valid_positions.len()
}

fn simulate_with_obstruction(
    map: &Vec<Vec<char>>,
    mut x: usize,
    mut y: usize,
    mut dir: Direction,
    visited: &mut HashSet<(usize, usize)>,
    path: &mut HashSet<((usize, usize), Direction)>,
) -> bool {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut step_count = 0;
    const MAX_STEPS: usize = 10000; // Arbitrary large number to limit steps.

    loop {
        // Exit if the step count exceeds the limit
        if step_count > MAX_STEPS {
            return false; // No loop detected within a reasonable step limit
        }
        step_count += 1;

        let (dx, dy) = dir.get_direction();

        // Check bounds
        if y as i32 + dy >= height || y as i32 + dy < 0 || x as i32 + dx >= width || x as i32 + dx < 0 {
            return false; // Guard escapes the map
        }

        let next_x = x as i32 + dx;
        let next_y = y as i32 + dy;

        // Check for obstacle
        if map[next_x as usize][next_y as usize] == '#' {
            dir = dir.rotate();
            continue;
        }

        // Move
        x = next_x as usize;
        y = next_y as usize;

        // Check for a loop
        if !path.insert(((x, y), dir.clone())) {
            return true; // Loop detected
        }

        // Mark as visited
        visited.insert((x, y));
    }
}

fn setup() -> Vec<Vec<char>> {
    let input = read_file("src/data/day6.txt");

    let height = input.len();
    let width = input[0].len();

    let mut grid = vec![vec![' '; width]; height];

    for i in 0..input.len() {
        let chars: Vec<char> = input[i].chars().collect();

        for j in 0..chars.len() {
            grid[i][j] = chars[j];
        }
    }

    grid
}

fn find_guard(map: &Vec<Vec<char>>) -> Option<(usize, usize, Direction)> {
    let guard_outfit = ['^', 'v', '<', '>'];

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if guard_outfit.contains(&map[row][col]) {
                return Some((row, col, Direction::new(&map[row][col])));
            }
        }
    }
    None
}

fn step(
    map: &Vec<Vec<char>>,
    mut x: usize,
    mut y: usize,
    mut dir: Direction,
    steps: i32,
    visited: &mut HashSet<(usize, usize)>,
) -> i32 {
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    loop {
        let (dx, dy) = dir.get_direction();

        // Check bounds and stop if out of bounds
        if y as i32 + dy >= height || y as i32 + dy < 0 || x as i32 + dx >= width || x as i32 + dx < 0 {
            break;
        }

        let next_x = x as i32 + dx;
        let next_y = y as i32 + dy;

        // Check the next position for an obstacle
        if map[next_x as usize][next_y as usize] == '#' {
            dir = dir.rotate();
            continue; // Retry with the new direction
        }

        // Move to the next position
        x = next_x as usize;
        y = next_y as usize;

        // Add the new position to visited
        visited.insert((x, y));
    }

    visited.len() as i32
}

