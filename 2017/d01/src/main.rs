use std::fs;

fn main() {
    println!("Part 1: {}", solve(false));
    println!("Part 2: {}", solve(true));
}

fn solve(two: bool) -> u32 {
    let input: String = fs::read_to_string("input").unwrap();
    let digits: Vec<u8> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let mut count = 0;

    let add: usize = if two { digits.len() / 2 } else { 1 };

    for i in 0..digits.len() {
        if digits[i] == digits[(i + add) % digits.len()] {
            count += digits[i] as u32;
        }
    }

    count
}
