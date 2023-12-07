use std::{cmp::Ordering, fs};

fn main() {
    let (p1, p2) = solve("input.in");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(filename: &str) -> (i64, i64) {
    let input = fs::read_to_string(filename).unwrap();
    let times: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let dists: Vec<i64> = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let time2 = combine_numbers(&times);
    let dist2 = combine_numbers(&dists);

    let mut result = (1, 1);

    for i in 0..times.len() {
        result.0 *= count(times[i], dists[i]);
    }

    result.1 = count(time2, dist2);

    result
}

fn count(time: i64, dist: i64) -> i64 {
    let t = time as f64;
    let d = dist as f64;

    let a = (t - (t.powi(2) - 4.0 * d).sqrt()) / 2.0;
    let b = (t + (t.powi(2) - 4.0 * d).sqrt()) / 2.0;

    let delta = 0.00000001;

    let on_margin = if (a - a.round()).abs() < delta {
        true
    } else {
        false
    };

    let a = a.ceil() as i64;
    let b = b.floor() as i64;

    if on_margin {
        b - a - 1
    } else {
        b - a + 1
    }
}

fn combine_numbers(numbers: &[i64]) -> i64 {
    let mut result = String::new();
    for number in numbers {
        result.push_str(&number.to_string());
    }
    result.parse::<i64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve("input_test.in").0, 288);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve("input_test.in").1, 71503);
    }
}
