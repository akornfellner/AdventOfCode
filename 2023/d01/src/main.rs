use std::fs;

fn main() {
    let input = fs::read_to_string("input.in").unwrap();
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
        p1 += p1_digits[0] * 10 + p1_digits[p1_digits.len() - 1];
        p2 += p2_digits[0] * 10 + p2_digits[p2_digits.len() - 1];
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
