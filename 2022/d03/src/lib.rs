use std::fs;

pub fn solve(input: &str, two: bool) -> i32 {
    let input = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;

    if !two {
        for line in lines {
            sum += calc_line(line);
        }
    } else {
        for i in (0..lines.len()).step_by(3) {
            sum += calc_three(lines[i], lines[i + 1], lines[i + 2]);
        }
    }

    sum
}

fn calc_line(line: &str) -> i32 {
    let l = line.len() / 2;
    let first = &line[..l];
    let second = &line[l..];

    let signs = intersect(first, second);

    value(signs.chars().next().unwrap())
}

fn calc_three(first: &str, second: &str, third: &str) -> i32 {
    let first = intersect(first, second);
    let signs = intersect(&first, third);

    value(signs.chars().next().unwrap())
}

fn intersect(first: &str, second: &str) -> String {
    let mut signs = String::new();

    for c in first.chars() {
        if second.contains(c) {
            signs.push(c)
        }
    }

    signs
}

fn value(sign: char) -> i32 {
    let x = sign as i32;

    if x > 90 {
        x - ('a' as i32) + 1
    } else {
        x - ('A' as i32) + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_works() {
        let result = solve("input_test.txt", false);
        assert_eq!(result, 157);
    }

    #[test]
    fn two_works() {
        let result = solve("input_test.txt", true);
        assert_eq!(result, 70);
    }
}
