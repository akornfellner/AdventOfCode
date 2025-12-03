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

    for line in input.lines() {
        let mut max = 0;
        let mut index = 0;

        for i in 0..line.len() - 1 {
            let number = line.chars().nth(i).unwrap() as usize - '0' as usize;
            if number > max {
                max = number;
                index = i;
            }
        }

        let mut max2 = 0;

        for i in (index + 1)..line.len() {
            let number = line.chars().nth(i).unwrap() as usize - '0' as usize;
            if number > max2 {
                max2 = number;
            }
        }

        p1 += max * 10 + max2;
    }

    (p1, 0)
}

fn find_max(s: &str, level: usize) -> (usize, usize) {
    let mut max = 0;
    let mut index = 0;

    for i in 0..s.len() - level - 1 {
        let number = s.chars().nth(i).unwrap() as usize - '0' as usize;
        if number > max {
            max = number;
            index = i;
        }
    }

    (max, index)
}
