use std::fs;

fn main() {
    println!("Part one: {}", solve("input.in", false));
    println!("Part two: {}", solve("input.in", true));
}

fn solve(input: &str, two: bool) -> i32 {
    let input = fs::read_to_string(input).unwrap();
    let mut numbers: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let mut count = 0;
    let mut index = 0;

    loop {
        let jump = numbers[index];
        if two && jump >= 3 {
            numbers[index] -= 1;
        } else {
            numbers[index] += 1;
        }
        index = (index as i32 + jump) as usize;
        count += 1;
        if index >= numbers.len() {
            break;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        assert_eq!(solve("input_test.in", false), 5);
    }

    #[test]
    fn test_solve_two() {
        assert_eq!(solve("input_test.in", true), 10);
    }
}
