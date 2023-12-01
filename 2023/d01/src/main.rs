use std::{collections::HashMap, fs};

fn main() {
    println!("Part one: {}", solve_one("input.in"));
    println!("Part two: {}", solve_two("input.in"));
}

fn solve_one(input: &str) -> i32 {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;

    for line in lines {
        let mut digits: Vec<char> = vec![];
        for c in line.chars() {
            if c.is_ascii_digit() {
                digits.push(c);
            }
        }
        let first = String::from(digits[0]);
        digits.reverse();
        let second = String::from(digits[0]);
        let number = first + &second;
        sum += number.parse::<i32>().unwrap();
    }

    sum
}

fn solve_two(input: &str) -> i32 {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let digits: HashMap<&str, char> = [
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]
    .iter()
    .cloned()
    .collect();

    let mut sum = 0;

    for line in lines {
        let mut number: Vec<char> = vec![];
        let chars: Vec<char> = line.chars().collect();

        for i in 0..line.len() {
            if chars[i].is_ascii_digit() {
                number.push(chars[i]);
            }
            for j in i + 1..line.len() {
                let word = &line[i..=j];
                if digits.contains_key(word) {
                    number.push(*digits.get(word).unwrap());
                }
            }
        }

        let first = String::from(number[0]);
        number.reverse();
        let second = String::from(number[0]);
        let number = first + &second;
        sum += number.parse::<i32>().unwrap();
    }

    sum
}
