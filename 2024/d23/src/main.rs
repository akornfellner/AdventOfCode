use std::{collections::HashSet, env::args};
use stopwatch::time;

#[time]
fn main() {
    let filename = args().nth(1).unwrap_or("input_test.txt".to_string());
    let (p1, p2) = solve(&filename);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();

    let mut edges = HashSet::new();
    let mut nodes = HashSet::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('-').collect();

        let a = parts[0].trim();
        let b = parts[1].trim();
        edges.insert((a.to_string(), b.to_string()));
        edges.insert((b.to_string(), a.to_string()));
        nodes.insert(a.to_string());
        nodes.insert(b.to_string());
    }

    let nodes: Vec<String> = nodes.into_iter().collect();

    let circles = find_circles(&nodes, &edges, 3);

    let mut p1 = 0;

    for circle in &circles {
        if has_t(circle) {
            p1 += 1;
        }
    }

    (p1, 0)
}

fn find_circles(nodes: &[String], edges: &HashSet<(String, String)>, k: usize) -> Vec<Vec<String>> {
    let mut results = Vec::new();
    let mut current_stack = Vec::new();
    get_circle(nodes, edges, k, 0, &mut current_stack, &mut results);
    results
}

fn get_circle(
    nodes: &[String],
    edges: &HashSet<(String, String)>,
    k: usize,
    start: usize,
    current_stack: &mut Vec<String>,
    results: &mut Vec<Vec<String>>,
) {
    if current_stack.len() == k {
        results.push(current_stack.clone());
        return;
    }

    if start >= nodes.len() {
        return;
    }

    for i in start..nodes.len() {
        let candidate = &nodes[i];

        let is_valid = current_stack
            .iter()
            .all(|n| edges.contains(&(n.clone(), candidate.clone())));

        if is_valid {
            current_stack.push(candidate.clone());

            get_circle(nodes, edges, k, i + 1, current_stack, results);

            current_stack.pop();
        }
    }
}

fn has_t(nodes: &[String]) -> bool {
    for node in nodes {
        if node.starts_with('t') {
            return true;
        }
    }
    false
}
