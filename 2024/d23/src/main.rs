use std::{collections::HashSet, env::args};
use stopwatch::time;

#[time]
fn main() {
    let filename = args().nth(1).unwrap_or("input_test.txt".to_string());
    let (p1, p2) = solve(&filename);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, String) {
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

    let circles = find_circles(&nodes, &edges, 3, false);

    let p1 = circles.iter().filter(|c| has_t(c)).count();

    let circles = find_circles(&nodes, &edges, 0, true);

    let mut max_circle = circles.iter().max_by_key(|c| c.len()).unwrap().to_vec();

    max_circle.sort();
    let p2 = max_circle.join(",");

    (p1, p2)
}

fn find_circles(
    nodes: &[String],
    edges: &HashSet<(String, String)>,
    k: usize,
    two: bool,
) -> Vec<Vec<String>> {
    let mut results = vec![];
    let mut current_stack = vec![];
    get_circle(nodes, edges, k, 0, &mut current_stack, &mut results, two);
    results
}

fn get_circle(
    nodes: &[String],
    edges: &HashSet<(String, String)>,
    k: usize,
    start: usize,
    current_stack: &mut Vec<String>,
    results: &mut Vec<Vec<String>>,
    two: bool,
) {
    if !two && current_stack.len() == k {
        results.push(current_stack.clone());
        return;
    }

    if start >= nodes.len() {
        return;
    }

    let mut found = false;

    for i in start..nodes.len() {
        let candidate = &nodes[i];

        let is_valid = current_stack
            .iter()
            .all(|n| edges.contains(&(n.clone(), candidate.clone())));

        if is_valid {
            current_stack.push(candidate.clone());

            get_circle(nodes, edges, k, i + 1, current_stack, results, two);

            current_stack.pop();

            found = true;
        }
    }

    if !found && two {
        results.push(current_stack.clone());
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
