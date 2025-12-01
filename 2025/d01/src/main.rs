use std::env::args;

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

    let mut current = 50;
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let value: isize = line[1..].parse().unwrap();
        let (a, b) = next_value(current, direction, value);
        current = a;
        p2 += b;
        if current == 0 {
            p1 += 1;
        }
    }

    (p1, p2)
}

fn next_value(current: isize, direction: char, value: isize) -> (isize, usize) {
    let mut zeros = value / 100;
    let value = value % 100;
    let new_value;

    match direction {
        'R' => {
            new_value = (current + value) % 100;
            if new_value < current {
                zeros += 1;
            }
        }
        'L' => {
            new_value = if value > current {
                if current != 0 {
                    zeros += 1;
                }
                (100 - value + current) % 100
            } else {
                current - value
            };
            if new_value == 0 && value != 0 {
                zeros += 1;
            }
        }
        _ => panic!("Invalid direction"),
    }

    (new_value, zeros as usize)
}
