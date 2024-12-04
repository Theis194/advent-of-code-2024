use crate::read_file;
use std::collections::HashMap;

pub fn result() {
    let result1 = solution1();
    println!("Result: {}", result1);
    let result2 = solution2();
    println!("Result: {}", result2);
}

fn solution1() -> i64 {
    let lines = read_file("src/data/day1.txt");

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split("   ").collect();

        left.push(parse(parts[0]));
        right.push(parse(parts[1]));
    }

    left.sort();
    right.sort();

    let mut accumulator: i64 = 0;
    for i in 0 .. left.len() {
        accumulator += (left[i] - right[i]).abs();
    }

    accumulator
}

fn solution2() -> i64 {
    let lines = read_file("src/data/day1.txt");

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split("   ").collect();

        left.push(parse(parts[0]));
        right.push(parse(parts[1]));
    }

    let mut apperences: HashMap<i64, i64> = HashMap::new();

    for num in left {
        apperences.entry(num).or_insert(0);
    }

    for num in right {
        apperences.entry(num).and_modify(|value| {*value += 1});
    }

    let mut accumulator = 0;
    for (key, value) in apperences {
        accumulator += key * value;
    }

    accumulator
}

fn parse(num: &str) -> i64 {
    num.parse::<i64>().expect("Failed to parse the input")
}