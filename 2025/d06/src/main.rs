use std::{env::args, vec};

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

    let operations: Vec<char> = input
        .lines()
        .last()
        .unwrap()
        .chars()
        .filter(|x| *x == '+' || *x == '*')
        .collect();

    let mut numbers = vec![];

    for line in input.lines().take(input.lines().count() - 1) {
        let nums: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        numbers.push(nums);
    }

    let mut p1 = 0;

    for col in 0..numbers[0].len() {
        let mut add = if operations[col] == '+' { 0 } else { 1 };
        for n in &numbers {
            if operations[col] == '+' {
                add += n[col];
            } else {
                add *= n[col];
            }
        }
        p1 += add;
    }

    let mut chars = vec![];

    for line in input.lines().take(input.lines().count() - 1) {
        let c: Vec<char> = line.chars().collect();
        chars.push(c);
    }

    let mut numbers2 = vec![];

    let mut col = 0;

    for y in 0..chars[0].len() {
        let mut number = 0;
        for x in &chars {
            let c = x[y];
            if c.is_ascii_digit() {
                number = number * 10 + c.to_digit(10).unwrap() as usize;
            }
        }
        if number != 0 {
            if numbers2.len() <= col {
                numbers2.push(vec![]);
            }
            numbers2[col].push(number);
        } else {
            col += 1;
        }
    }

    let mut p2 = 0;

    for x in 0..numbers2.len() {
        let mut add = if operations[x] == '+' { 0 } else { 1 };
        for y in 0..numbers2[x].len() {
            if operations[x] == '+' {
                add += numbers2[x][y];
            } else {
                add *= numbers2[x][y];
            }
        }
        p2 += add;
    }

    (p1, p2)
}
