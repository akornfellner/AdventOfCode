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
        let mut last_index = 0;
        let mut sum = 0;

        for level in 0..2 {
            let level = 2 - level - 1;
            let (max, new_last_index) = find_max(&line[last_index..], level);
            sum += max as u64 * (10_u64).pow(level.try_into().unwrap());
            last_index += new_last_index + 1;
        }

        p1 += sum as usize;

        let mut sum = 0;
        let mut last_index = 0;

        for level in 0..12 {
            let level = 12 - level - 1;
            let (max, new_last_index) = find_max(&line[last_index..], level);
            sum += max as u64 * (10_u64).pow(level.try_into().unwrap());
            last_index += new_last_index + 1;
        }

        p2 += sum as usize;
    }

    (p1, p2)
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
