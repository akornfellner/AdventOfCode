use std::{
    collections::{HashMap, HashSet},
    env::args,
};
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

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let mut patterns = parts[0]
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let designs = parts[1]
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut possible: HashSet<String> = HashSet::new();
    let mut not_possible: HashSet<String> = HashSet::new();

    patterns.sort_by_key(|b| std::cmp::Reverse(b.len()));

    let mut pos_designs = vec![];

    let mut p1 = 0;

    for design in &designs {
        if check(design, &patterns, &mut possible, &mut not_possible) {
            p1 += 1;
            pos_designs.push(design.to_string());
        }
    }

    let mut p2 = 0;
    let mut dp: HashMap<String, usize> = HashMap::new();

    for design in pos_designs {
        p2 += count(&design, &patterns, &mut dp);
    }

    (p1, p2)
}

fn check(
    design: &str,
    patterns: &[String],
    possible: &mut HashSet<String>,
    not_possible: &mut HashSet<String>,
) -> bool {
    if possible.contains(design) {
        return true;
    }
    if not_possible.contains(design) {
        return false;
    }

    let mut result = false;

    for pattern in patterns {
        if pattern.len() > design.len() {
            continue;
        }
        if pattern.len() == design.len() {
            if pattern == design {
                result = true;
                break;
            }
            continue;
        }
        if design.starts_with(pattern)
            && check(&design[pattern.len()..], patterns, possible, not_possible)
        {
            result = true;
        }
    }

    if result {
        possible.insert(design.to_string());
    } else {
        not_possible.insert(design.to_string());
    }

    result
}

fn count(design: &str, patterns: &[String], dp: &mut HashMap<String, usize>) -> usize {
    if dp.contains_key(design) {
        return dp[design];
    }

    let mut result = 0;

    for pattern in patterns {
        if pattern.len() > design.len() {
            continue;
        }
        if pattern.len() == design.len() {
            if pattern == design {
                result += 1;
            }
            continue;
        }
        if design.starts_with(pattern) {
            result += count(&design[pattern.len()..], patterns, dp);
        }
    }

    dp.insert(design.to_string(), result);

    result
}
