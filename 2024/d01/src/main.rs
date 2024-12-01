use std::collections::HashMap;

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (i32, i32) {
    let input = std::fs::read_to_string(filename).unwrap();

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    left.sort();
    right.sort();

    let p1 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    let mut counts: HashMap<i32, i32> = HashMap::new();
    for &value in &right {
        *counts.entry(value).or_insert(0) += 1;
    }

    let mut p2 = 0;

    for value in left {
        if let Some(&count) = counts.get(&value) {
            p2 += value * count
        }
    }

    (p1, p2)
}
