use regex::Regex;

use crate::read_file_string;

pub fn result() {
    let result1 = solution1();
    println!("Result: {}", result1);
    let result2 = solution2();
    println!("Result: {}", result2);
}

fn solution1() -> i32 {
    let input = read_file_string("src/data/day3.txt");
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut total = 0;

    for capture in regex.captures_iter(&input) {
        let x: i32 = capture[1].parse().unwrap();
        let y: i32 = capture[2].parse().unwrap();

        total += x * y;
    }
    
    total
}

fn solution2() -> i32 {
    let input = read_file_string("src/data/day3.txt");
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();

    let mut total = 0;
    let mut mul_enabled = true;

    for capture in regex.captures_iter(&input) {
        if let Some(mul_match) = capture.get(1) {
            if mul_enabled {
                let x: i32 = capture[1].parse().unwrap();
                let y: i32 = capture[2].parse().unwrap();
        
                total += x * y;
            }
        } else if let Some(do_match) = capture.get(0) {
            if do_match.as_str() == "do()" {
                mul_enabled = true;
            } else if do_match.as_str() == "don't()" {
                mul_enabled = false;
            }
        }
    }
    
    total
}
