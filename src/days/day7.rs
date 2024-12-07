use crate::read_file;

pub fn result() {
    let result1 = solution1();
    println!("Result: {}", result1);
    let result2 = solution2();
    println!("Result: {}", result2);
}

fn solution1() -> i64 {
    let equations = setup();

    equations
        .iter()
        .map(|equation| {
            if let Some((first, rest)) = equation.1.split_first() {
                if can_reach_target(equation.0, rest, *first) {
                    return equation.0;
                }
            }
            0
        })
        .sum()
}

fn solution2() -> i64 {
    let equations = setup();
    
    equations
        .iter()
        .map(|equation| {
            if let Some((first, rest)) = equation.1.split_first() {
                if can_reach_target_with_concat(equation.0, rest, *first) {
                    return equation.0;
                }
            }
            0
        })
        .sum()
}

fn setup() -> Vec<(i64, Vec<i64>)> {
    let data: Vec<String> = read_file("src/data/day7.txt");

    let mut input: Vec<(i64, Vec<i64>)> = Vec::new();
    for line in data {
        let parts: (&str, &str) = line.split_once(":").expect("failed to split");
        let test_value = parts.0.parse::<i64>().expect("Failed to parse test value");

        let values: Vec<i64> = parts.1.trim().split_whitespace().map(|s| s.parse::<i64>().expect("Failed to parse value")).collect();

        input.push((test_value, values));
    }

    input
}

fn can_reach_target(
    test_value: i64,
    numbers: &[i64],
    current_value: i64,
) -> bool {
    if numbers.is_empty()  {
        return current_value == test_value;
    }

    let num = numbers[0];
    let remaining_numbers = &numbers[1..];

    if can_reach_target(test_value, remaining_numbers, current_value + num) {
        return true;
    }

    if can_reach_target(test_value, remaining_numbers, current_value * num) {
        return true;
    }

    false
}

fn can_reach_target_with_concat(
    test_value: i64,
    numbers: &[i64],
    current_value: i64,
) -> bool {
    if numbers.is_empty()  {
        return current_value == test_value;
    }

    let num = numbers[0];
    let remaining_numbers = &numbers[1..];

    if can_reach_target_with_concat(test_value, remaining_numbers, current_value + num) {
        return true;
    }

    if can_reach_target_with_concat(test_value, remaining_numbers, current_value * num) {
        return true;
    }

    if can_reach_target_with_concat(test_value, remaining_numbers, concat_nums(current_value, num)) {
        return true;
    }

    false
}

fn concat_nums(first: i64, second: i64) -> i64 {
    let concat = first.to_string() + &second.to_string();

    concat.parse::<i64>().expect("Failed to concat numbers")
}