use std::fs;

pub fn solve_one(input: &str) -> usize {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;

    for line in lines {
        let code = line.len();
        let mut string_count = code - 2;

        let chars: Vec<char> = line.chars().collect();

        let mut i = 0;

        while i < chars.len() {
            if chars[i] == '\\' {
                match chars[i + 1] {
                    'x' => string_count -= 3,
                    '\\' => {
                        string_count -= 1;
                        i += 1;
                    }
                    _ => string_count -= 1,
                }
            }
            i += 1
        }

        sum += code - string_count;
    }

    sum
}

pub fn solve_two(input: &str) -> usize {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;

    for line in lines {
        let mut count = 2usize; //always for the parenthesis at the beginning and the end

        for c in line.chars().collect::<Vec<char>>() {
            match c {
                '\\' | '"' => count += 2,
                _ => count += 1,
            }
        }

        sum += count - line.len();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve_one("input_test.txt");
        assert_eq!(result, 12);
    }

    #[test]
    fn two_works() {
        let result = solve_two("input_test.txt");
        assert_eq!(result, 19);
    }
}
