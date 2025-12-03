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

    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        p1 += get_joltage(line, 2);
        p2 += get_joltage(line, 12);
    }

    (p1, p2)
}

fn get_joltage(line: &str, number: usize) -> usize {
    let mut last_index = 0;
    let mut sum = 0;

    for level in 0..number {
        let level = number - level - 1;
        let (max, new_last_index) = find_max(&line[last_index..], level);
        sum += max as u64 * (10_u64).pow(level.try_into().unwrap());
        last_index += new_last_index + 1;
    }

    sum as usize
}

fn find_max(s: &str, level: usize) -> (usize, usize) {
    let mut max = 0;
    let mut index = 0;

    for i in 0..s.len() - level {
        let number = s.chars().nth(i).unwrap() as usize - '0' as usize;
        if number > max {
            max = number;
            index = i;
        }
    }

    (max, index)
}
