use std::collections::{hash_map, HashMap, HashSet, VecDeque};

use crate::read_file_split;

pub fn result() {
    let result1 = solution1();
    println!("Result: {}", result1);
    let result2 = solution2();
    println!("Result: {}", result2);
}

fn solution1() -> i32 {
    let (rules, updates) = setup();

    find_middle_pages(&updates, &rules)
}

fn solution2() -> i32 {
    let (rules, updates) = setup();

    reorder_and_sum_middle_pages(&updates, &rules)
}

fn setup() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let (rules, updates) = read_file_split("src/data/day5.txt");

    let rules: Vec<(i32, i32)> = rules
        .iter()
        .flat_map(|s| {
            s.split_once('|').map(|(p1, p2)| {
                (
                    p1.parse::<i32>().expect("Failed to parse p1"),
                    p2.parse::<i32>().expect("Failed to parse p2"),
                )
            })
        })
        .collect();

    let updates: Vec<Vec<i32>> = updates
        .iter()
        .map(|s| {
            s.split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn build_graph(
    updates: &[i32],
    rules: &[(i32, i32)],
) -> (HashMap<i32, Vec<i32>>, HashMap<i32, usize>) {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    let update_set: HashSet<i32> = updates.iter().cloned().collect();

    for &node in updates {
        in_degree.entry(node).or_insert(0);
    }

    for &(x, y) in rules {
        if update_set.contains(&x) && update_set.contains(&y) {
            graph.entry(x).or_default().push(y);
            *in_degree.entry(y).or_insert(0) += 1;
        }
    }

    (graph, in_degree)
}

fn is_vaild_update(update: &[i32], rules: &[(i32, i32)]) -> bool {
    let (graph, mut in_degree) = build_graph(update, rules);

    let mut queue: VecDeque<i32> = in_degree
        .iter()
        .filter(|&(_, &degree)| degree == 0)
        .map(|(&node, _)| node)
        .collect();

    let mut sorted_order = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted_order.push(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    sorted_order == update
}

fn find_middle_pages(updates: &[Vec<i32>], rules: &[(i32, i32)]) -> i32 {
    updates
        .iter()
        .filter_map(|update| {
            if is_vaild_update(update, rules) {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn reorder_and_sum_middle_pages(updates: &[Vec<i32>], rules: &[(i32, i32)]) -> i32 {
    updates
        .iter()
        .filter_map(|update| {
            if !is_vaild_update(update, rules) {
                topological_sort(update, rules).map(|sorted_update| {
                    sorted_update[sorted_update.len() / 2]
                })
            } else {
                None
            }
        })
        .sum()
}

fn topological_sort(update: &Vec<i32>, rules: &[(i32, i32)]) -> Option<Vec<i32>> {
    // Create adjacency list and in-degree map only for pages in the update
    let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, usize> = HashMap::new();

    // Initialize in-degree and adjacency list based on the rules
    for &(before, after) in rules {
        // Only consider rules where both pages are in the update
        if update.contains(&before) && update.contains(&after) {
            adjacency_list.entry(before).or_insert(vec![]).push(after);
            *in_degree.entry(after).or_insert(0) += 1;
            in_degree.entry(before).or_insert(0); // Ensure 'before' has an entry with 0 in-degree
        }
    }

    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut sorted: Vec<i32> = Vec::new();

    // Add all nodes (pages) with in-degree 0 to the queue
    for &page in update.iter() {
        if let Some(&degree) = in_degree.get(&page) {
            if degree == 0 {
                queue.push_back(page);
            }
        }
    }

    // Process the queue
    while let Some(page) = queue.pop_front() {
        sorted.push(page);

        // Update the in-degree of neighbors
        if let Some(neighbors) = adjacency_list.get(&page) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    // Check if the sort included all nodes in the update
    if sorted.len() == update.len() {
        Some(sorted)
    } else {
        None // Graph has a cycle
    }
}
