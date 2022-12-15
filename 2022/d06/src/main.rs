use std::{collections::HashSet, fs};

fn main() {
    println!("part one: {}", solve("input.txt", false));
    println!("part two: {}", solve("input.txt", true));
}

fn solve(input: &str, two: bool) -> usize {
    let chars: Vec<char> = fs::read_to_string(input).unwrap().chars().collect();

    let size = if two { 14 } else { 4 };

    let mut result = 0;

    for i in size - 1..chars.len() {
        let set: HashSet<_> = chars[i + 1 - size..=i].iter().collect();

        if set.len() >= size {
            result = i + 1;
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", false);
        assert_eq!(result, 7);
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", true);
        assert_eq!(result, 19);
    }
}
