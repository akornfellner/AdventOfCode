use std::fs;

fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(input: &str) -> (i32, i32) {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let names = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut p1 = 0;
    let mut p2 = 0;

    for line in lines {
        let mut p1_digits: Vec<i32> = vec![];
        let mut p2_digits: Vec<i32> = vec![];
        let chars: Vec<char> = line.chars().collect();

        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                p1_digits.push(c as i32 - '0' as i32);
                p2_digits.push(c as i32 - '0' as i32);
            } else {
                for (n, name) in names.iter().enumerate() {
                    if chars[i..].starts_with(&name.chars().collect::<Vec<char>>()) {
                        p2_digits.push(n as i32 + 1);
                    }
                }
            }
        }
        p1 += get_number(&mut p1_digits);
        p2 += get_number(&mut p2_digits);
    }

    (p1, p2)
}

fn get_number(digits: &mut [i32]) -> i32 {
    let first = digits[0];
    digits.reverse();
    let second = digits[0];
    first * 10 + second
}
