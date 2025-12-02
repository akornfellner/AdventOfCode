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

    let ranges = input.split(',').collect::<Vec<&str>>();

    let mut p1 = 0;
    let mut p2 = 0;

    for range in ranges {
        let bounds = range
            .split('-')
            .map(|x| x.trim())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        for number in bounds[0]..=bounds[1] {
            if check_invalid(number) {
                p1 += number;
            }
            if check_invalid2(number) {
                p2 += number;
            }
        }
    }

    (p1, p2)
}

fn check_invalid(number: usize) -> bool {
    let number = number.to_string();
    let part1 = &number[0..number.len() / 2];
    let part2 = &number[number.len() / 2..];

    part1 == part2
}

fn check_invalid2(number: usize) -> bool {
    let number = number.to_string();
    let l = number.len();

    for split in 1..=(l / 2) {
        let parts = number
            .chars()
            .collect::<Vec<char>>()
            .chunks(split)
            .map(|x| x.iter().collect::<String>())
            .collect::<Vec<String>>();

        let mut result = true;
        for i in 1..parts.len() {
            if parts[0] != parts[i] {
                result = false;
                break;
            }
        }
        if result {
            return true;
        }
    }

    false
}
