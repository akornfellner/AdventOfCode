use std::env::args;
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

    let mut polymer = input.clone();

    let p1 = get_length(&polymer);

    let mut p2 = p1;

    for c in 'a'..='z' {
        polymer = polymer
            .replace(&c.to_string(), "")
            .replace(&c.to_ascii_uppercase().to_string(), "");
        let length = get_length(&polymer);
        if length < p2 {
            p2 = length;
        }
        polymer = input.clone(); // Reset polymer for the next iteration
    }

    (p1, p2)
}

fn is_pair(a: char, b: char) -> bool {
    (a.is_lowercase() && b.is_uppercase() && a.to_ascii_uppercase() == b)
        || (a.is_uppercase() && b.is_lowercase() && a.to_ascii_lowercase() == b)
}

fn get_length(polymer: &str) -> usize {
    let mut finished = false;
    let mut polymer = polymer.to_string();

    while !finished {
        finished = true;
        let mut new_polymer = String::new();
        let mut skip_next = false;

        for (i, c) in polymer.chars().enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }

            if i < polymer.len() - 1 && is_pair(c, polymer.chars().nth(i + 1).unwrap()) {
                skip_next = true;
                finished = false;
            } else {
                new_polymer.push(c);
            }
        }

        polymer = new_polymer;
    }

    polymer.len()
}
