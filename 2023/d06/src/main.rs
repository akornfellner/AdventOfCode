use std::fs;

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

    let mut time2 = String::new();
    for time in &times {
        time2.push_str(&time.to_string());
    }
    let time2 = time2.parse::<i64>().unwrap();

    let mut dist2 = String::new();
    for dist in &dists {
        dist2.push_str(&dist.to_string());
    }
    let dist2 = dist2.parse::<i64>().unwrap();

    let mut result = (1, 1);

    for i in 0..times.len() {
        let time = times[i];
        let dist = dists[i];

        let mut count = 0;

        for v in 0..=time {
            let s = (time - v) * v;
            if s > dist {
                count += 1;
            }
        }

        result.0 *= count;
    }

    let mut count = 0;

    for v in 0..=time2 {
        let s = (time2 - v) * v;
        if s > dist2 {
            count += 1;
        }
    }

    result.1 *= count;

    result
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
