use std::collections::HashMap;

fn main() {
    let (p1, p2) = solve("input.txt");
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

fn solve(filename: &str) -> (usize, usize) {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();

    let numbers = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut seen = HashMap::new();

    let p1 = numbers
        .iter()
        .map(|x| count(*x, 25, &mut seen))
        .sum::<usize>();

    let p2 = numbers
        .iter()
        .map(|x| count(*x, 75, &mut seen))
        .sum::<usize>();

    (p1, p2)
}

fn count(number: usize, rounds: usize, seen: &mut HashMap<(usize, usize), usize>) -> usize {
    let result;
    if rounds == 0 {
        result = 1;
    } else if seen.contains_key(&(number, rounds)) {
        result = seen[&(number, rounds)];
    } else if number == 0 {
        result = count(1, rounds - 1, seen);
    } else if number.to_string().len() % 2 != 0 {
        result = count(number * 2024, rounds - 1, seen);
    } else {
        let (first, second) = split_number(number);
        result = count(first, rounds - 1, seen) + count(second, rounds - 1, seen);
    }
    seen.insert((number, rounds), result);
    result
}

fn split_number(number: usize) -> (usize, usize) {
    let number_s = number.to_string();
    let (first, second) = number_s.split_at(number_s.len() / 2);
    (first.parse().unwrap(), second.parse().unwrap())
}
