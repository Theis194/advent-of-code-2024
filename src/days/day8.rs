use std::collections::{HashMap, HashSet};

use crate::read_file;

pub fn result() {
    let result1 = solution1();
    println!("Result: {}", result1);
    let result2 = solution2();
    println!("Result: {}", result2);
}

fn solution1() -> i32 {
    let (antennas, width, height) = setup();

    let antinode_positions = calculate_antinodes(&antennas, width as i32, height as i32);

    antinode_positions.len() as i32
}

fn solution2() -> i32 {
    let (antennas, width, height) = setup();

    let antinode_positions = calculate_antinodes_resonant(&antennas, width as i32, height as i32);

    //println!("{:?}", antinode_positions);

    antinode_positions.len() as i32
}

fn setup() -> (HashMap<char, Vec<(i32, i32)>>, usize, usize) {
    let input = read_file("src/data/day8.txt");

    let height = input.len();
    let width = input[0].len();

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, row) in input.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch != '.' {
                antennas
                    .entry(ch)
                    .or_insert_with(Vec::new)
                    .push((x as i32, y as i32));
            }
        }
    }

    (antennas, width, height)
}

fn calculate_antinodes(
    antennas: &HashMap<char, Vec<(i32, i32)>>,
    width: i32,
    height: i32,
) -> HashSet<(i32, i32)> {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_freq, positions) in antennas {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue; // Skip the same antenna
                }

                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                let mx1 = 2 * x1 - x2;
                let my1 = 2 * y1 - y2;
                let mx2 = 2 * x2 - x1;
                let my2 = 2 * y2 - y1;


                if mx1 >= 0 && my1 >= 0 && mx1 < width && my1 < height {
                    antinodes.insert((mx1, my1));
                }

                if mx2 >= 0 && my2 >= 0 && mx2 < width && my2 < height {
                    antinodes.insert((mx2, my2));
                }
            }
        }
    }

    antinodes
}

fn calculate_antinodes_resonant(
    antennas: &HashMap<char, Vec<(i32, i32)>>,
    width: i32,
    height: i32,
) -> HashSet<(i32, i32)> {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (freq, positions) in antennas {

        let position_count = positions.len();

        if position_count < 2 {
            continue;
        }

        for i in 0..position_count {
            let (x1, y1) = positions[i];

            for j in 0..position_count {
                if i == j {
                    continue;
                }

                let (x2, y2) = positions[j];
                let dx = x2 - x1;
                let dy = y2 - y1;

                let gcd = gcd(dx, dy);
                let direction = (dx / gcd, dy / gcd);

                let mut x = x1 - direction.0;
                let mut y = y1 - direction.1;
                while x >= 0 && y >= 0 && x < width && y < height {
                    if !positions.contains(&(x, y)) {
                        antinodes.insert((x, y));
                    }
                    x -= direction.0;
                    y -= direction.1;
                }

                let mut x = x1 + direction.0;
                let mut y = y1 + direction.1;
                while x >= 0 && y >= 0 && x < width && y < height {
                    if !positions.contains(&(x, y)) {
                        antinodes.insert((x, y));
                    }
                    x += direction.0;
                    y += direction.1;
                }
            }

            antinodes.insert((x1, y1));
        }
    }

    antinodes
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

