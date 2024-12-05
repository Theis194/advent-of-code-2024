use std::fs::{self, File};
use std::io::{BufRead, BufReader};

pub fn read_file(file: &str) -> Vec<String> {
    let file = File::open(file).expect("Failed to open file!");

    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        lines.push(line);
    }

    lines
}

pub fn read_file_string(file: &str) -> String {
    fs::read_to_string(file).expect("Unable to read file")
}

pub fn read_file_split(file: &str) -> (Vec<String>, Vec<String>) {
    let input= fs::read_to_string(file).expect("Unable to read file");

    let parts: Vec<String> = input
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let first = parts[0]
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    
    let second = parts[1]
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    (first, second)
}