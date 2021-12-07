use std::fs;

pub fn solve(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).unwrap();

    let numbers: Vec<i32> = input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let max = *numbers.iter().max().unwrap();
    let min = *numbers.iter().min().unwrap();

    let mut sum: i32;
    let mut lastsum = std::i32::MAX;

    for i in min..=max {
        sum = 0;
        for n in &numbers {
            let diff = i - n;
            sum += sum_to(diff.abs());
        }
        if sum < lastsum {
            lastsum = sum;
        }
    }

    lastsum
}

fn sum_to(n: i32) -> i32 {
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn it_works() {
        assert_eq!(solve("input_test.txt"), 168);
    }
}
