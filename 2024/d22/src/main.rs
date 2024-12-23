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

    let numbers: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();

    let p1 = numbers.iter().map(|x| calculate(*x, 2000)).sum();

    let mut max_bananas = HashMap::new();

    for number in numbers.iter() {
        let mut n = *number;
        let mut old = (n % 10) as isize;
        let mut diffs = [0, 0, 0, 0];
        let mut bananas = HashMap::new();
        for i in 1..=2000 {
            n = step(n);
            let d = (n % 10) as isize;
            let diff = d - old;
            old = d;
            diffs = [diffs[1], diffs[2], diffs[3], diff];

            if i > 3 && !bananas.contains_key(&diffs) {
                bananas.insert(diffs, d);
            }
        }

        for (key, value) in bananas {
            let entry = max_bananas.entry(key).or_insert(0);
            *entry += value;
        }
    }

    let p2 = *max_bananas.values().max().unwrap();

    (p1, p2.try_into().unwrap())
}

fn step(number: usize) -> usize {
    let mut n = number << 6;
    n ^= number;
    n &= 16777215;
    let number = n;

    let mut n = number.checked_shr(5).unwrap_or(0);
    n ^= number;
    n &= 16777215;
    let number = n;

    let mut n = number << 11;
    n ^= number;
    n &= 16777215;

    n
}

fn calculate(number: usize, steps: usize) -> usize {
    let mut n = number;
    for _ in 0..steps {
        n = step(n);
    }
    n
}
