use std::{collections::HashMap, env::args};
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

    patterns.sort_by_key(|b| std::cmp::Reverse(b.len()));

    let mut p1 = 0;
    let mut p2 = 0;
    let mut dp: HashMap<String, usize> = HashMap::new();

    for design in &designs {
        let c = count(design, &patterns, &mut dp);
        if c > 0 {
            p1 += 1;
        }
        p2 += c;
    }

    (p1, p2)
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
