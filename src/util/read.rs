use std::fs::File;
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