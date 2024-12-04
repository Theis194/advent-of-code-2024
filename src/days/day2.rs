use crate::read_file;

pub fn result() {
    let result1 = solution1();
    println!("Result: {}", result1);
    let result2 = solution2();
    println!("Result: {}", result2);
}

fn solution1() -> i32 {
    let reports = read_file("src/data/day2.txt");

    let mut safe_report_count = 0;
    for report in reports {
        let report: Vec<i32> = report
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Failed to parse"))
            .collect();

        if is_safe(&report, 0, |curr, next| -> bool {curr > next}) {
            safe_report_count += 1;
        } else if is_safe(&report, 0, |curr, next| -> bool {curr < next}) {
            safe_report_count += 1;
        }
    }

    safe_report_count
}

fn solution2() -> i32 {
    let reports = read_file("src/data/day2.txt");

    let mut safe_report_count = 0;
    for report in reports {
        let report: Vec<i32> = report
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Failed to parse"))
            .collect();

        if is_safe(&report, 0, |curr, next| curr > next)
            || is_safe(&report, 0, |curr, next| curr < next)
            || is_safe_with_dampner(&report, |curr, next| curr > next)
            || is_safe_with_dampner(&report, |curr, next| curr < next) {
            safe_report_count += 1;
        }
    }

    safe_report_count
}

fn is_safe(report: &Vec<i32>, current: i32, condition: fn(i32, i32) -> bool) -> bool {
    if current + 1 >= report.len() as i32 {
        return true;
    }

    if condition(report[current as usize], report[(current + 1) as usize]) {
        let distance = (report[current as usize] - report[(current + 1) as usize]).abs();
        if distance >= 1 && distance <= 3 {
            return is_safe(report, current + 1, condition);
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn is_safe_with_dampner(report: &Vec<i32>, condition: fn(i32, i32) -> bool) -> bool {
    for i in 0..report.len() {
        // Try removing one element and check if the report becomes safe
        let mut new_report = report.clone();
        new_report.remove(i);

        if is_safe(&new_report, 0, condition) {
            return true;
        }
    }

    false
}